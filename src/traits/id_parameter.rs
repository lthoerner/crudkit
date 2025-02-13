/// A trait that allows queries including an ID field to use unique nomenclature if desired.
///
/// The format for the URL will look like
/// `https://fixwise.io/some/record/endpoint?id_parameter_name=123456`. If the ID parameter is just
/// named `id`, simply use [`GenericIdParameter`].
pub trait IdParameter {
    /// Create the parameter with an inner [`usize`].
    fn new(value: usize) -> Self;
    /// Get the inner [`usize`] ID parameter.
    fn id(&self) -> usize;
}
