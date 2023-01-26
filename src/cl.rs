use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct MsgLock<T: Keeper> {
    state: AtomicUsize,
    cell: UnsafeCell<T>,
    inbox: crossbeam::queue::SegQueue<T::Update>,
}

impl<T: Keeper> MsgLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicUsize::new(AccessGuard::<T>::STATE_INIT_VALUE),
            cell: UnsafeCell::new(value),
            inbox: crossbeam::queue::SegQueue::new(),
        }
    }

    #[inline]
    pub fn update(&self, message: T::Update) {
        AccessGuard::<_, 0>::execute_or_enqueue(self, message);
    }

    #[inline]
    pub fn try_lock(&self) -> Option<impl DerefMut<Target = T> + '_> {
        AccessGuard::<_, 0>::try_exclusive(self)
    }

    #[inline]
    pub async fn lock(&self) -> impl DerefMut<Target = T> + '_ {
        loop {
            match self.try_lock() {
                Some(guard) => return guard,
                None => tokio::task::yield_now().await,
            }
        }
    }

    #[inline]
    pub fn lock_blocking(&self) -> impl DerefMut<Target = T> + '_ {
        loop {
            match self.try_lock() {
                Some(guard) => return guard,
                None => std::thread::yield_now(),
            }
        }
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        AccessGuard::<T, 0>::apply_pending(self);
        self.cell.get_mut()
    }
}

unsafe impl<T: Keeper> Send for MsgLock<T> where T: Send {}

unsafe impl<T: Keeper> Sync for MsgLock<T> where T: Sync {}

pub trait Keeper {
    type Update;

    fn apply(&mut self, cmd: Self::Update);
}

struct AccessGuard<'a, T: Keeper, const OFFSET: usize = 0>(&'a MsgLock<T>);

impl<'a, T: Keeper + 'a, const OFFSET: usize> AccessGuard<'a, T, OFFSET> {
    // this allows one invalid blind decrement without underflow
    const STATE_INIT_VALUE: usize = 0;
    const VALUE_RANGE: usize = usize::MAX >> 1;
    const EXCLUSIVE_FLAG: usize = !Self::VALUE_RANGE;

    #[inline]
    fn try_exclusive(lock: &'a MsgLock<T>) -> Option<Self> {
        // try raise the exclusive flag
        let result = lock.state.fetch_or(Self::EXCLUSIVE_FLAG, Ordering::SeqCst);
        if result & Self::EXCLUSIVE_FLAG == 0 {
            // no one is accessing the lock therefore returning the guard is valid
            Some({
                let mut guard = Self(lock);
                let pending = result & Self::VALUE_RANGE;
                for _ in 0..pending {
                    // decrement it on every iteration to be able to recover from panics
                    lock.state.fetch_sub(1, Ordering::SeqCst);
                    guard.apply_update();
                }
                guard
            })
        } else {
            // the lock is already accessed exclusively, do _not_ reset the flag that was raised by
            // the other guard!
            None
        }
    }

    #[inline]
    fn execute_or_enqueue(lock: &'a MsgLock<T>, message: T::Update) {
        // promise the message
        lock.state.fetch_add(1, Ordering::SeqCst);
        lock.inbox.push(message);
        drop(AccessGuard::<T, 0>::try_exclusive(lock));
    }

    #[inline]
    fn apply_pending(lock: &'a mut MsgLock<T>) {
        let pending = lock.state.load(Ordering::Relaxed);
        for _ in 0..pending {
            lock.state.fetch_sub(1, Ordering::SeqCst);
            ManuallyDrop::new(Self(lock)).apply_update();
        }
    }

    #[inline]
    fn apply_update(&mut self) {
        let lock = self.0;
        // try again until the message has been received
        loop {
            if let Some(message) = lock.inbox.pop() {
                self.deref_mut().apply(message);
                break;
            } else {
                std::thread::yield_now();
            }
        }
    }
}

impl<T: Keeper, const OFFSET: usize> Deref for AccessGuard<'_, T, OFFSET> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: while this ReadGuard exists, the keeper can be accessed
        unsafe { NonNull::new_unchecked(self.0.cell.get()).as_ref() }
    }
}

impl<T: Keeper, const OFFSET: usize> DerefMut for AccessGuard<'_, T, OFFSET> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: while this ReadGuard exists, the keeper can be accessed
        unsafe { NonNull::new_unchecked(self.0.cell.get()).as_mut() }
    }
}

impl<'a, T: Keeper + 'a, const OFFSET: usize> Drop for AccessGuard<'a, T, OFFSET> {
    fn drop(&mut self) {
        // reset the exclusive flag
        let _ = self.0.state.fetch_and(Self::VALUE_RANGE, Ordering::SeqCst);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    use std::thread::available_parallelism;

    struct IncrementInstruction;

    struct Hoard(usize);

    impl Keeper for Hoard {
        type Update = IncrementInstruction;

        #[inline]
        fn apply(&mut self, _: Self::Update) {
            self.0 += 1;
        }
    }

    #[test]
    pub fn concurrent_increment() {
        const INSTRUCTIONS_PER_THREAD: usize = 1_000;
        let thread_count: usize =
            available_parallelism().map(NonZeroUsize::get).unwrap_or(1) * 1000;

        let mut hoard = MsgLock::new(Hoard(0));

        std::thread::scope(|s| {
            for _ in 0..thread_count {
                s.spawn(|| {
                    for _ in 0..INSTRUCTIONS_PER_THREAD {
                        hoard.update(IncrementInstruction);
                    }
                });
            }
        });

        assert_eq!(thread_count * INSTRUCTIONS_PER_THREAD, hoard.get_mut().0);
    }

    #[test]
    pub fn concurrent_increment_with_lock() {
        const LOCK_INCREMENT: usize = 1000;
        const INSTRUCTIONS_PER_THREAD: usize = 1_000;
        let thread_count: usize =
            available_parallelism().map(NonZeroUsize::get).unwrap_or(1) * 1000;

        let mut hoard = MsgLock::new(Hoard(0));

        std::thread::scope(|s| {
            for _ in 0..thread_count {
                s.spawn(|| {
                    for _ in 0..INSTRUCTIONS_PER_THREAD {
                        hoard.update(IncrementInstruction);
                    }
                });
            }

            for _ in 0..LOCK_INCREMENT {
                let mut lock = hoard.lock_blocking();
                let before = lock.0;
                std::hint::black_box(&mut lock);
                lock.0 += 1;
                std::hint::black_box(&mut lock);
                let after = lock.0;
                assert_eq!(1, after - before);
            }
        });

        assert_eq!(
            thread_count * INSTRUCTIONS_PER_THREAD + LOCK_INCREMENT,
            hoard.get_mut().0
        );
    }

    #[test]
    pub fn increment_while_locked() {
        const INSTRUCTIONS: usize = 1_000_000;
        let mut hoard = MsgLock::new(Hoard(0));

        {
            let lock = hoard.lock_blocking();
            assert_eq!(0, lock.0);

            for _ in 0..INSTRUCTIONS {
                hoard.update(IncrementInstruction);
                assert_eq!(0, lock.0);
            }

            assert_eq!(0, lock.0);
            drop(lock);
        }

        assert_eq!(INSTRUCTIONS, hoard.get_mut().0);
    }

    #[test]
    pub fn in_order_while_locked() {
        const INSTRUCTIONS_PER_THREAD: usize = 1_000_000;

        let hoard = MsgLock::new(HoardAsc(0));

        struct HoardAsc(usize);

        impl Keeper for HoardAsc {
            type Update = usize;

            fn apply(&mut self, cmd: Self::Update) {
                if self.0 >= cmd {
                    panic!("ERROR ON: self.0={} >= cmd={}", self.0, cmd);
                } else {
                    self.0 = cmd;
                }
            }
        }

        std::thread::scope(|s| {
            s.spawn(|| {
                for i in 1..=INSTRUCTIONS_PER_THREAD {
                    hoard.update(i);
                }
            });

            for _ in 0..INSTRUCTIONS_PER_THREAD {
                let mut guard = hoard.lock_blocking();
                std::hint::black_box(&mut guard);
                assert!(guard.0 <= INSTRUCTIONS_PER_THREAD);
            }
        })
    }
}
