#[allow(unused_imports)]
use crate::prelude::*;


/// The `colored` crate uses OS specific features to colorize the output, which are not available in
/// the WASM target. This trait provides a mock implementation of the `colored` crate for the WASM target.
#[cfg(target_family = "wasm")]
pub trait MockColored {
    fn green(&self) -> String;
    fn cyan(&self) -> String;
    fn bold(&self) -> String;
}

#[cfg(target_family = "wasm")]
impl<T: AsRef<str>> MockColored for T {
    fn green(&self) -> String {
        self.as_ref().to_owned()
    }
    fn cyan(&self) -> String {
        self.as_ref().to_owned()
    }
    fn bold(&self) -> String {
        self.as_ref().to_owned()
    }
}

/// Serialize a map of regex objects to a map of regex strings.
#[cfg(feature = "serde")]
pub fn serialize_regex_map<S>(
    map: &IndexMap<RegexToken, Regex>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map_serializer = serializer.serialize_map(Some(map.len()))?;
    for (key, value) in map {
        map_serializer.serialize_entry(key, &value.to_string())?;
    }
    map_serializer.end()
}


/// Count the number of new lines in a slice.
pub fn count_new_lines(slice: &str) -> (usize, Option<usize>) {
    let mut offset_after_newline = None;
    let mut count = 0;
    for (offset, byte) in slice.bytes().enumerate() {
        if byte == b'\n' {
            offset_after_newline = Some(offset + 1);
            count += 1;
        }
    }
    (count, offset_after_newline)
}

/// Count column position of a char.
///
/// The offset is the byte offset of the character in the slice.
/// The resulting column position is the 1 indexed position of the utf-8 character in the slice.
pub fn count_col_position(slice: &str, last_newline_offset: usize, offset: usize) -> usize {
    slice[last_newline_offset..offset].chars().count() + 1
}
