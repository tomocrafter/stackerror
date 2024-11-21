//! Prelude module that exports commonly used types and traits.

pub use crate::error::{ErrorCode, ErrorStacks, StackError};
pub use crate::stack_msg;
pub type Result<T> = std::result::Result<T, StackError>;
