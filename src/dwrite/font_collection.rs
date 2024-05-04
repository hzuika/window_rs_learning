use windows::Win32::Graphics::DirectWrite::IDWriteFontCollection3;

use super::font_family::FontFamily;

pub struct FontCollection(pub(super) IDWriteFontCollection3);

impl FontCollection {
    pub fn get_font_family_count(&self) -> usize {
        unsafe { self.0.GetFontFamilyCount() as _ }
    }

    pub fn get_font_family(&self, index: usize) -> anyhow::Result<FontFamily> {
        let family = unsafe { self.0.GetFontFamily(index as _) }?;
        Ok(FontFamily(family))
    }
}

pub struct FontCollectionIter<'a> {
    collection: &'a FontCollection,
    index: usize,
}

impl<'a> Iterator for FontCollectionIter<'a> {
    type Item = FontFamily;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.collection.get_font_family_count() {
            let family = self.collection.get_font_family(self.index);
            self.index += 1;
            match family {
                Ok(family) => Some(family),
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

impl<'a> IntoIterator for &'a FontCollection {
    type Item = FontFamily;
    type IntoIter = FontCollectionIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            collection: self,
            index: 0,
        }
    }
}

impl<'a> ExactSizeIterator for FontCollectionIter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.collection.get_font_family_count()
    }
}
