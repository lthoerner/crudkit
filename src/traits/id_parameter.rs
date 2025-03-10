use serde::{Deserialize, Serialize};

/// A trait that allows queries including an ID field to use unique nomenclature if desired.
///
/// The format for the URL will look like
/// `https://fixwise.io/some/record/endpoint?id_parameter_name=123456`. If the ID parameter is just
/// named `id` and there are no other parameters needed, simply use [`GenericIdParameter`].
pub trait IdParameter: Send + Sync {
    /// Create the parameter with an inner [`usize`].
    fn new(value: usize) -> Self;
    /// Get the inner [`usize`] ID parameter.
    fn id(&self) -> usize;
}

/// A simple query parameter type to be used in handler functions if the only necessary parameter is
/// a numerical ID.
#[derive(Clone, Serialize, Deserialize)]
pub struct GenericIdParameter {
    id: usize,
}

impl IdParameter for GenericIdParameter {
    fn new(value: usize) -> Self {
        Self { id: value }
    }

    fn id(&self) -> usize {
        self.id
    }
}
