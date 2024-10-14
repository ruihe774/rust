use crate::sync::atomic::AtomicU32;
use crate::sys::futex::{futex_wait, futex_wake, futex_wake_all};
use crate::time::Duration;

pub struct Requeuer;

impl Requeuer {
    #[inline]
    pub const fn new() -> Requeuer {
        Requeuer
    }

    #[inline]
    pub fn wake_one(&self, expected: u32, from: &AtomicU32) {
        let _ = expected;
        futex_wake(from);
    }

    #[inline]
    pub fn wake_one_and_requeue_other(&self, expected: u32, from: &AtomicU32) {
        let _ = expected;
        futex_wake_all(from);
    }

    #[inline]
    pub fn wait_requeuable(
        &self,
        expected: u32,
        from: &AtomicU32,
        timeout: Option<Duration>,
    ) -> bool {
        futex_wait(from, expected, timeout)
    }

    #[inline]
    pub unsafe fn wake_another(&self) {}
}
