use window_rs_learning::dwrite::{
    factory::{Factory, FactoryType},
    localized_strings::InformationalStringId,
};

fn main() -> anyhow::Result<()> {
    let factory = Factory::new(FactoryType::Isolated)?;
    let collection = factory.get_system_font_collection(true)?;
    for family in &collection {
        for font in &family {
            if let Some(strings) =
                font.get_informational_strings(InformationalStringId::FullName)?
            {
                if let Some(string) = strings.get_string_by_locales(&["ja-jp", "en-us"])? {
                    println!("{}", string);
                }
            }
        }
    }

    Ok(())
}
