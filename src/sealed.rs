/// Supertrait of all traits which must not be implemented outside of
/// this crate. As this trait is not nameable from outside of this crate
/// those traits can be extended without breaking compatibility.
pub trait Sealed {}
