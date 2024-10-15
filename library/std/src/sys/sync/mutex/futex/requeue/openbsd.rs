use crate::sync::atomic::AtomicU32;
use crate::sys::futex::{futex_requeue, futex_wait, futex_wake};
use crate::sys::sync::Mutex;
use crate::time::Duration;

pub struct Requeuer(Mutex);

impl Requeuer {
    #[inline]
    pub const fn new() -> Requeuer {
        Requeuer(Mutex::new())
    }

    #[inline]
    pub fn wake_one(&self, expected: u32, from: &AtomicU32) {
        let _ = expected;
        futex_wake(from);
    }

    #[inline]
    pub fn wake_one_and_requeue_other(&self, expected: u32, from: &AtomicU32) {
        let _ = expected;
        futex_requeue(from, &self.0.futex);
    }

    #[inline]
    pub fn wait_requeuable(
        &self,
        expected: u32,
        from: &AtomicU32,
        timeout: Option<Duration>,
    ) -> bool {
        let r = futex_wait(from, expected, timeout);
        if r {
            self.0.lock();
            unsafe { self.0.mark_contended() };
        }
        r
    }

    #[inline]
    pub unsafe fn wake_another(&self) {
        unsafe { self.0.unlock() };
    }
}
