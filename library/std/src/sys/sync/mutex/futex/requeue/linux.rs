use crate::sync::atomic::AtomicU32;
use crate::sys::pi_futex as pi;
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
        let _ = pi::futex_requeue(from, expected, &self.0.futex, false)
            .expect("failed to requeue futex waiters");
    }

    #[inline]
    pub fn wake_one_and_requeue_other(&self, expected: u32, from: &AtomicU32) {
        let _ = pi::futex_requeue(from, expected, &self.0.futex, true)
            .expect("failed to requeue futex waiters");
    }

    #[inline]
    pub fn wait_requeuable(
        &self,
        expected: u32,
        from: &AtomicU32,
        timeout: Option<Duration>,
    ) -> bool {
        pi::futex_wait_requeue(from, expected, &self.0.futex, timeout)
            .expect("failed to wait on futex requeuably")
    }

    #[inline]
    pub unsafe fn wake_another(&self) {
        unsafe { self.0.unlock() }
    }
}
