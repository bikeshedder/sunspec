/// Every group and model implements this trait.
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait Group: crate::sealed::Sealed + Sized {
    /// Length of this group including nested groups. Repeating groups
    /// whose count is not fixed are not included, so for groups
    /// containing them this is the minimum length.
    const LEN: u16;
}
