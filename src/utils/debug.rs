use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn enable_debug() {
    DEBUG_ENABLED.store(true, Ordering::SeqCst);
}

pub fn debug_println(args: std::fmt::Arguments) {
    if DEBUG_ENABLED.load(Ordering::SeqCst) {
        println!("[DEBUG] {}", args);
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::utils::debug::debug_println(format_args!($($arg)*));
    })
}
