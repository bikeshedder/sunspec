/// Every group and model implements this trait.
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait Group: crate::sealed::Sealed + Sized {
    /// Group length (without nested and repeating groups)
    const LEN: u16;
    /// Information about this group and its fields which is also
    /// available at runtime
    const GROUP_INFO: GroupInfo;
}

/// Information about a group which is known at runtime.
///
/// Every group provides this information via [`Group::GROUP_INFO`].
/// The information about the root group of a model is available via
/// [`ModelInfo::group`](crate::ModelInfo::group).
#[derive(Debug)]
pub struct GroupInfo {
    /// Name of the group as defined by the SunSpec specification
    pub name: &'static str,
    /// Label of the group as defined by the SunSpec specification
    pub label: &'static str,
    /// Description of the group as defined by the SunSpec specification
    pub description: &'static str,
    /// Points and nested groups of this group in the order they are
    /// defined by the SunSpec specification. Padding points are omitted.
    pub fields: &'static [FieldInfo],
}

/// Information about a field of a group which is known at runtime.
#[derive(Debug)]
pub struct FieldInfo {
    /// Name of the field in the generated struct. This is also the
    /// name used when serializing a group with the `serde` feature.
    pub name: &'static str,
    /// Label of the field as defined by the SunSpec specification
    pub label: &'static str,
    /// Description of the field as defined by the SunSpec specification
    pub description: &'static str,
    /// Whether this field is a point or a nested group
    pub kind: FieldKind,
}

/// The kind of a field of a group.
#[derive(Debug)]
pub enum FieldKind {
    /// A point
    Point,
    /// A nested group
    Group(&'static GroupInfo),
    /// A repeating group which is represented as `Vec`
    RepeatingGroup(&'static GroupInfo),
}
