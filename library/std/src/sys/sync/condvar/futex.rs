use crate::sync::atomic::AtomicU32;
use crate::sync::atomic::Ordering::Relaxed;
use crate::sys::sync::{Mutex, Requeuer};
use crate::time::Duration;

pub struct Condvar {
    // The value of this atomic is simply incremented on every notification.
    // This is used by `.wait()` to not miss any notifications after
    // unlocking the mutex and before waiting for notifications.
    futex: AtomicU32,
    requeuer: Requeuer,
}

impl Condvar {
    #[inline]
    pub const fn new() -> Self {
        Self { futex: AtomicU32::new(0), requeuer: Requeuer::new() }
    }

    // All the memory orderings here are `Relaxed`,
    // because synchronization is done by unlocking and locking the mutex.

    pub fn notify_one(&self) {
        let expected = self.futex.fetch_add(1, Relaxed).wrapping_add(1);
        self.requeuer.wake_one(expected, &self.futex);
    }

    pub fn notify_all(&self) {
        let expected = self.futex.fetch_add(1, Relaxed).wrapping_add(1);
        // Waking all: may cause thundering-herd formation.
        // So we instead wake one and requeue remaining.
        self.requeuer.wake_one_and_requeue_other(expected, &self.futex);
    }

    pub unsafe fn wait(&self, mutex: &Mutex) {
        self.wait_optional_timeout(mutex, None);
    }

    pub unsafe fn wait_timeout(&self, mutex: &Mutex, timeout: Duration) -> bool {
        self.wait_optional_timeout(mutex, Some(timeout))
    }

    unsafe fn wait_optional_timeout(&self, mutex: &Mutex, timeout: Option<Duration>) -> bool {
        // Examine the notification counter _before_ we unlock the mutex.
        let futex_value = self.futex.load(Relaxed);

        // Unlock the mutex before going to sleep.
        mutex.unlock();

        // Wait, but only if there hasn't been any
        // notification since we unlocked the mutex.
        let r = self.requeuer.wait_requeuable(futex_value, &self.futex, timeout);

        // Lock the mutex again.
        mutex.lock();

        if r {
            // This also wakes one requeued waiter.
            // And that waiter will then wake next.
            self.requeuer.wake_another();
        }

        r
    }
}
