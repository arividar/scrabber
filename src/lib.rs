// #![feature(absolute_path)]

// Modules
pub mod is_empty;
pub mod mock_screenshot_writer;
pub mod screenshot_writer;

// Re-exports
pub use screenshot_writer::{ScreenshotWriter, ScreenshotError};
pub use is_empty::is_image_empty;

// Re-export the mock implementation for testing
#[cfg(test)]
pub use mock_screenshot_writer::MockScreenshotWriter;
