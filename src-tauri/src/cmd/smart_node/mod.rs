pub mod pool;
pub mod settings;
pub mod speed_test;
pub mod switch;
pub mod health;
pub mod group;
pub mod history;
pub mod dashboard;

// Re-export all command functions
pub use pool::*;
pub use settings::*;
pub use speed_test::*;
pub use switch::*;
pub use health::*;
pub use group::*;
pub use history::*;
pub use dashboard::*;
