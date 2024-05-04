use windows::Win32::{Foundation::BOOL, Graphics::DirectWrite::IDWriteFont3};

use super::localized_strings::{InformationalStringId, LocalizedStrings};

pub struct Font(pub(crate) IDWriteFont3);

impl Font {
    pub fn get_informational_strings(
        &self,
        id: InformationalStringId,
    ) -> anyhow::Result<Option<LocalizedStrings>> {
        let mut exists = BOOL::from(false);
        let mut strings = None;
        unsafe {
            self.0
                .GetInformationalStrings(id.into(), &mut strings, &mut exists)
        }?;
        if exists.as_bool() {
            match strings {
                Some(strings) => Ok(Some(LocalizedStrings(strings))),
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
