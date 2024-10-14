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
        self.0.lock();
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
    pub unsafe fn wake_another(&self) {
        self.0.wake();
    }
}
