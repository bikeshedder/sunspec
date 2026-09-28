/// Every group and model implements this trait.
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait Group: crate::sealed::Sealed + Sized {
    /// Group length (without nested and repeating groups)
    const LEN: u16;
}
