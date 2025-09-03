//! Output formatting and display logic
//! 
//! This module handles all test result formatting and display concerns,
//! separating them from the core test execution logic in TestRunner.

pub mod formatter;
pub mod summary;

pub use formatter::*;
pub use summary::*;