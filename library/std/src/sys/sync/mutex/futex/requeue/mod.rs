cfg_if::cfg_if! {
    if #[cfg(all(any(target_os = "linux", target_os = "android"), not(miri)))] {
        mod linux;
        pub use linux::Requeuer;
    } else if #[cfg(target_os = "openbsd")] {
        mod openbsd;
        pub use openbsd::Requeuer;
    } else {
        mod fallback;
        pub use fallback::Requeuer;
    }
}
