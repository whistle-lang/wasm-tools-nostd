use alloc::{borrow::Cow, vec::Vec};

use crate::{encoding_size, Encode, Section, SectionId};

/// A custom section holding arbitrary data.
#[derive(Clone, Debug)]
pub struct CustomSection<'a> {
    /// The name of this custom section.
    pub name: Cow<'a, str>,
    /// This custom section's data.
    pub data: Cow<'a, [u8]>,
}

impl Encode for CustomSection<'_> {
    fn encode(&self, sink: &mut Vec<u8>) {
        let encoded_name_len = encoding_size(u32::try_from(self.name.len()).unwrap());
        (encoded_name_len + self.name.len() + self.data.len()).encode(sink);
        self.name.encode(sink);
        sink.extend(&*self.data);
    }
}

impl Section for CustomSection<'_> {
    fn id(&self) -> u8 {
        SectionId::Custom.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_section() {
        let custom = CustomSection {
            name: "test".into(),
            data: Cow::Borrowed(&[11, 22, 33, 44]),
        };

        let mut encoded = Vec::<u8>::new();
        custom.encode(&mut encoded);

        let mut compare_to = Vec::<u8>::new();
        compare_to.extend(&[9, 4, b't', b'e', b's', b't', 11, 22, 33, 44]);
        assert_eq!(encoded, compare_to);
    }
}
