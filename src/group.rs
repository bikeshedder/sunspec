/// Static metadata for a field inside a SunSpec group.
#[derive(Clone, Copy, Debug)]
pub struct FieldInfo {
    /// Stable serialized field name.
    pub name: &'static str,
    /// Human-readable label.
    pub label: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Whether this field is a point or a nested group.
    pub kind: FieldKind,
}

/// Static metadata for a SunSpec group.
#[derive(Clone, Copy, Debug)]
pub struct GroupInfo {
    /// Stable group name from the source JSON.
    pub name: &'static str,
    /// Human-readable group label.
    pub label: &'static str,
    /// Human-readable group description.
    pub description: &'static str,
    /// Group fields in source order.
    pub fields: &'static [FieldInfo],
}

/// The kind of a generated group field.
#[derive(Clone, Copy, Debug)]
pub enum FieldKind {
    /// A regular point field.
    Point,
    /// A nested single group.
    Group(fn() -> &'static GroupInfo),
    /// A repeating group.
    RepeatingGroup(fn() -> &'static GroupInfo),
}

/// Every group and model implements this trait.
pub trait Group: Sized {
    /// Group length (without nested and repeating groups)
    const LEN: u16;
}

/// Runtime type information for generated groups and models.
pub trait GroupMeta: Group {
    /// Static metadata for this group.
    fn group_info() -> &'static GroupInfo;
}
