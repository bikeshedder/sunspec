use std::ops::Range;

/// Panic free slicing function inspired by
/// https://stackoverflow.com/a/40206905/994342
pub(crate) fn get_slice<T>(slice: &[T], range: Range<usize>) -> Option<&[T]> {
    if range.start > range.end || range.end > slice.len() {
        None
    } else {
        Some(&slice[range])
    }
}
