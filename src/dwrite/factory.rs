use std::any::type_name;

use anyhow::Context;
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteFactory3, IDWriteFactory6, IDWriteFactory7,
    IDWriteFontCollection, IDWriteFontCollection1, DWRITE_FACTORY_TYPE,
    DWRITE_FACTORY_TYPE_ISOLATED, DWRITE_FACTORY_TYPE_SHARED,
};

use windows::core::Interface;

use super::font_collection::FontCollection;
use super::font_family::FontFamilyModel;

pub struct Factory(IDWriteFactory7);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FactoryType {
    Isolated,
    Shared,
}

impl Into<DWRITE_FACTORY_TYPE> for FactoryType {
    fn into(self) -> DWRITE_FACTORY_TYPE {
        match self {
            FactoryType::Isolated => DWRITE_FACTORY_TYPE_ISOLATED,
            FactoryType::Shared => DWRITE_FACTORY_TYPE_SHARED,
        }
    }
}

impl Factory {
    pub fn new(factory_type: FactoryType) -> anyhow::Result<Self> {
        let factory = unsafe { DWriteCreateFactory(factory_type.into())? };
        Ok(Factory(factory))
    }

    pub fn get_system_font_collection(
        &self,
        check_for_updates: bool,
    ) -> anyhow::Result<FontCollection> {
        let mut collection = None;
        let factory: &IDWriteFactory = &self.0.cast()?;
        unsafe { factory.GetSystemFontCollection(&mut collection, check_for_updates) }?;
        let collection = collection
            .with_context(|| format!("{} is none", type_name::<IDWriteFontCollection>()))?;
        Ok(FontCollection(collection.cast()?))
    }

    pub fn get_system_font_collection1(
        &self,
        include_downloadable_fonts: bool,
        check_for_updates: bool,
    ) -> anyhow::Result<FontCollection> {
        let mut collection = None;
        let factory: &IDWriteFactory3 = &self.0.cast()?;
        unsafe {
            factory.GetSystemFontCollection(
                include_downloadable_fonts,
                &mut collection,
                check_for_updates,
            )
        }?;
        let collection = collection
            .with_context(|| format!("{} is none", type_name::<IDWriteFontCollection1>()))?;
        Ok(FontCollection(collection.cast()?))
    }

    pub fn get_system_font_collection2(
        &self,
        include_downloadable_fonts: bool,
        font_family_model: FontFamilyModel,
    ) -> anyhow::Result<FontCollection> {
        let factory: &IDWriteFactory6 = &self.0.cast()?;
        let collection = unsafe {
            factory.GetSystemFontCollection(include_downloadable_fonts, font_family_model.into())
        }?;
        Ok(FontCollection(collection.cast()?))
    }

    pub fn get_system_font_collection3(
        &self,
        include_downloadable_fonts: bool,
        font_family_model: FontFamilyModel,
    ) -> anyhow::Result<FontCollection> {
        let factory: &IDWriteFactory7 = &self.0.cast()?;
        let collection = unsafe {
            factory.GetSystemFontCollection(include_downloadable_fonts, font_family_model.into())
        }?;
        Ok(FontCollection(collection.cast()?))
    }
}
