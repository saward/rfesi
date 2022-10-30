//! Module for easy imports.

pub use crate::builders::EsiBuilder;
pub use crate::client::{Esi, RequestType};
pub use crate::errors::{EsiError, EsiResult};
pub(crate) use serde::Deserialize;

/// Response from the ESI API, containing metadata and the specific data
pub struct Data<T> {
    /// How many pages of data available, if any
    pub pages: Option<usize>,
    /// Specific data
    pub value: T,
}
