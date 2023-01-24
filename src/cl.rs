use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct MsgLock<T: Keeper> {
    state: AtomicUsize,
    cell: UnsafeCell<T>,
    inbox: crossbeam::queue::SegQueue<T::Message>,
}

impl<T: Keeper> MsgLock<T> {
    pub const fn new(hoard: T) -> Self {
        Self {
            state: AtomicUsize::new(AccessGuard::<T>::STATE_INIT_VALUE),
            cell: UnsafeCell::new(hoard),
            inbox: crossbeam::queue::SegQueue::new(),
        }
    }

    #[inline]
    pub fn update(&self, instruction: T::Message) {
        AccessGuard::<_, 0>::execute_or_enqueue(self, instruction);
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
        self.cell.get_mut()
    }
}

unsafe impl<T: Keeper> Send for MsgLock<T> where T: Send {}

unsafe impl<T: Keeper> Sync for MsgLock<T> where T: Sync {}

pub trait Keeper {
    type Message;

    fn send(&mut self, cmd: Self::Message);
}

struct AccessGuard<'a, T: Keeper, const OFFSET: usize = 0>(&'a MsgLock<T>);

impl<'a, T: Keeper + 'a, const OFFSET: usize> AccessGuard<'a, T, OFFSET> {
    // this allows one invalid blind decrement without underflow
    const STATE_INIT_VALUE: usize = 1;
    const VALUE_RANGE: usize = usize::MAX >> 1;
    const EXCLUSIVE_FLAG: usize = !Self::VALUE_RANGE;

    #[inline]
    fn try_exclusive(lock: &'a MsgLock<T>) -> Option<Self> {
        // try raise the exclusive flag
        if lock.state.fetch_or(Self::EXCLUSIVE_FLAG, Ordering::SeqCst) & Self::EXCLUSIVE_FLAG == 0 {
            // no one is accessing the lock therefore returning the guard is valid
            Some(Self(lock))
        } else {
            // the lock is already accessed exclusively, do _not_ reset the flag that was raised by
            // the other guard!
            None
        }
    }

    fn execute_or_enqueue(lock: &'a MsgLock<T>, instruction: T::Message) {
        // promise the instruction
        lock.state.fetch_add(1, Ordering::SeqCst);
        if let Some(mut guard) = AccessGuard::<T, 1>::try_exclusive(lock) {
            // execute directly
            guard.deref_mut().send(instruction);
        } else {
            // failed to lock, but
            // by incrementing the state, an instruction was promised... deliver it!
            lock.inbox.push(instruction);
        }
    }

    /// Returns the state value (without exclusive flag)
    fn follow_all_instructions_from_queue(&mut self) -> bool {
        let lock = self.0;
        loop {
            // ignore the exclusive flag which might be set due to locking attempts.
            match (lock.state.fetch_sub(1, Ordering::SeqCst) & Self::VALUE_RANGE) - OFFSET {
                // invalid blind decrement, needs to be fixed
                0 => break true,
                // init value reached, no fixing required
                1 => {
                    debug_assert_eq!(1, Self::STATE_INIT_VALUE);
                    break false;
                }
                // there is data in the queue
                _ => {
                    // try again until the instruction has been received
                    loop {
                        if let Some(instruction) = lock.inbox.pop() {
                            self.deref_mut().send(instruction);
                            break;
                        }
                    }
                }
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
        let fix_invalid_blind_read = self.follow_all_instructions_from_queue();
        // reset the exclusive flag
        let _ = self.0.state.fetch_sub(
            if fix_invalid_blind_read {
                Self::EXCLUSIVE_FLAG - 1
            } else {
                Self::EXCLUSIVE_FLAG
            },
            Ordering::SeqCst,
        );
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
        type Message = IncrementInstruction;

        #[inline]
        fn send(&mut self, _: Self::Message) {
            self.0 += 1;
        }
    }

    #[test]
    pub fn concurrent_increment() {
        const INSTRUCTIONS_PER_THREAD: usize = 10_000;
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
        const INSTRUCTIONS_PER_THREAD: usize = 10_000;
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
}
