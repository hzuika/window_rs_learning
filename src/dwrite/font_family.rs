use windows::Win32::Graphics::DirectWrite::{
    IDWriteFontFamily2, DWRITE_FONT_FAMILY_MODEL, DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC,
    DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE,
};

use super::font::Font;

pub enum FontFamilyModel {
    TYPOGRAPHIC,
    WeightStretchStyle,
}

impl Into<DWRITE_FONT_FAMILY_MODEL> for FontFamilyModel {
    fn into(self) -> DWRITE_FONT_FAMILY_MODEL {
        match self {
            FontFamilyModel::TYPOGRAPHIC => DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC,
            FontFamilyModel::WeightStretchStyle => DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE,
        }
    }
}

pub struct FontFamily(pub(super) IDWriteFontFamily2);

impl FontFamily {
    pub fn get_font_count(&self) -> usize {
        unsafe { self.0.GetFontCount() as _ }
    }

    pub fn get_font(&self, index: usize) -> anyhow::Result<Font> {
        let font = unsafe { self.0.GetFont(index as _) }?;
        Ok(Font(font))
    }
}

pub struct FontFamilyIter<'a> {
    family: &'a FontFamily,
    index: usize,
}

impl<'a> Iterator for FontFamilyIter<'a> {
    type Item = Font;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.family.get_font_count() {
            let font = self.family.get_font(self.index);
            self.index += 1;
            match font {
                Ok(font) => Some(font),
                Err(e) => {
                    eprintln!("{}", e);
                    assert!(false);
                    None
                }
            }
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a FontFamily {
    type Item = Font;
    type IntoIter = FontFamilyIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            family: self,
            index: 0,
        }
    }
}

impl<'a> ExactSizeIterator for FontFamilyIter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.family.get_font_count() as usize
    }
}
