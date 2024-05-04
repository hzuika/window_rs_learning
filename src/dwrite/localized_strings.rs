use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::BOOL,
        Graphics::DirectWrite::{
            IDWriteLocalizedStrings, DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE,
            DWRITE_INFORMATIONAL_STRING_DESCRIPTION, DWRITE_INFORMATIONAL_STRING_DESIGNER,
            DWRITE_INFORMATIONAL_STRING_DESIGNER_URL,
            DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG,
            DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL, DWRITE_INFORMATIONAL_STRING_FULL_NAME,
            DWRITE_INFORMATIONAL_STRING_ID, DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION,
            DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL, DWRITE_INFORMATIONAL_STRING_MANUFACTURER,
            DWRITE_INFORMATIONAL_STRING_NONE, DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME,
            DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
            DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES,
            DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES,
            DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT,
            DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG,
            DWRITE_INFORMATIONAL_STRING_TRADEMARK,
            DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES,
            DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES,
            DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS,
            DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME,
            DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES,
            DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES,
            DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME,
        },
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InformationalStringId {
    None,
    CopyrightNotice,
    VersionStrings,
    Trademark,
    Manufacturer,
    Designer,
    DesignerUrl,
    Description,
    FontVendorUrl,
    LicenseDescription,
    LicenseInfoUrl,
    Win32FamilyNames,
    Win32SubfamilyNames,
    TypographicFamilyNames,
    TypographicSubfamilyNames,
    SampleText,
    FullName,
    PostscriptName,
    PostscriptCidName,
    WeightStretchStyleFamilyName,
    DesignScriptLanguageTag,
    SupportedScriptLanguageTag,
    PreferredFamilyNames,
    PreferredSubfamilyNames,
    WwsFamilyName,
}

impl Into<DWRITE_INFORMATIONAL_STRING_ID> for InformationalStringId {
    fn into(self) -> DWRITE_INFORMATIONAL_STRING_ID {
        match self {
            InformationalStringId::None => DWRITE_INFORMATIONAL_STRING_NONE,
            InformationalStringId::CopyrightNotice => DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE,
            InformationalStringId::VersionStrings => DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS,
            InformationalStringId::Trademark => DWRITE_INFORMATIONAL_STRING_TRADEMARK,
            InformationalStringId::Manufacturer => DWRITE_INFORMATIONAL_STRING_MANUFACTURER,
            InformationalStringId::Designer => DWRITE_INFORMATIONAL_STRING_DESIGNER,
            InformationalStringId::DesignerUrl => DWRITE_INFORMATIONAL_STRING_DESIGNER_URL,
            InformationalStringId::Description => DWRITE_INFORMATIONAL_STRING_DESCRIPTION,
            InformationalStringId::FontVendorUrl => DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL,
            InformationalStringId::LicenseDescription => {
                DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION
            }
            InformationalStringId::LicenseInfoUrl => DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL,
            InformationalStringId::Win32FamilyNames => {
                DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES
            }
            InformationalStringId::Win32SubfamilyNames => {
                DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES
            }
            InformationalStringId::TypographicFamilyNames => {
                DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES
            }
            InformationalStringId::TypographicSubfamilyNames => {
                DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES
            }
            InformationalStringId::SampleText => DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT,
            InformationalStringId::FullName => DWRITE_INFORMATIONAL_STRING_FULL_NAME,
            InformationalStringId::PostscriptName => DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
            InformationalStringId::PostscriptCidName => {
                DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME
            }
            InformationalStringId::WeightStretchStyleFamilyName => {
                DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME
            }
            InformationalStringId::DesignScriptLanguageTag => {
                DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG
            }
            InformationalStringId::SupportedScriptLanguageTag => {
                DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG
            }
            InformationalStringId::PreferredFamilyNames => {
                DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES
            }
            InformationalStringId::PreferredSubfamilyNames => {
                DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES
            }
            InformationalStringId::WwsFamilyName => DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME,
        }
    }
}

pub struct LocalizedStrings(pub(super) IDWriteLocalizedStrings);

impl LocalizedStrings {
    pub fn get_string(&self, index: usize) -> anyhow::Result<String> {
        let index = index as _;
        let len = unsafe { self.0.GetStringLength(index) }?;
        let mut buf = vec![0_u16; len as usize + 1];
        unsafe { self.0.GetString(index, buf.as_mut_slice()) }?;
        Ok(unsafe { PCWSTR::from_raw(buf.as_ptr()).to_string() }?)
    }

    pub fn get_string_by_locale(&self, locale: &str) -> anyhow::Result<Option<String>> {
        let index = self.find_locale_name(locale)?;
        match index {
            Some(index) => Ok(Some(self.get_string(index as _)?)),
            None => Ok(None),
        }
    }

    pub fn get_string_by_locales_with_fallback<F: FnOnce() -> anyhow::Result<Option<String>>>(
        &self,
        locales: &[&str],
        fallback: Option<F>,
    ) -> anyhow::Result<Option<String>> {
        for locale in locales {
            if let Some(string) = self.get_string_by_locale(locale)? {
                return Ok(Some(string));
            }
        }
        if let Some(fallback) = fallback {
            fallback()
        } else {
            Ok(None)
        }
    }

    pub fn get_string_by_locales(&self, locales: &[&str]) -> anyhow::Result<Option<String>> {
        let fallback = || self.get_string(0).map(Some);
        self.get_string_by_locales_with_fallback(locales, Some(fallback))
    }

    pub fn get_locale_name(&self, index: usize) -> anyhow::Result<String> {
        let index = index as _;
        let len = unsafe { self.0.GetLocaleNameLength(index) }?;
        let mut buf = vec![0_u16; len as usize + 1];
        unsafe { self.0.GetLocaleName(index, buf.as_mut_slice()) }?;
        Ok(unsafe { PCWSTR::from_raw(buf.as_ptr()).to_string() }?)
    }

    pub fn find_locale_name(&self, locale: &str) -> anyhow::Result<Option<u32>> {
        let mut index: u32 = 0;
        let mut exists = BOOL::from(false);
        unsafe {
            self.0
                .FindLocaleName(&HSTRING::from(locale), &mut index, &mut exists)
        }?;
        if exists.as_bool() {
            Ok(Some(index))
        } else {
            Ok(None)
        }
    }
}
