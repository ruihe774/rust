cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))] {
        mod pi;
        pub use pi::Mutex;
    } else {
        mod normal;
        pub use normal::Mutex;
    }
}

mod requeue;
pub use requeue::Requeuer;
