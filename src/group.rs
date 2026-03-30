/// Every group and model implements this trait.
pub trait Group: Sized {
    /// Group length (without nested and repeating groups)
    const LEN: u16;
}
