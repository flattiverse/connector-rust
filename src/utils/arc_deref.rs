use arc_swap::strategy::Strategy;
use arc_swap::Guard;
use std::ops::Deref;
use std::sync::Arc;

pub struct ArcStringDeref(pub Arc<String>);

impl Deref for ArcStringDeref {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

pub struct GuardedArcStringDeref<S: Strategy<Arc<String>>>(pub Guard<Arc<String>, S>);

impl<S: Strategy<Arc<String>>> Deref for GuardedArcStringDeref<S> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
