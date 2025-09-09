use crate::foundation::registry::{Hkey, HkeyData, HkeyPath};

pub fn create_classic_theme_registry() -> Hkey {
    let mut hkey = Hkey::new(HkeyPath::new(
        r"HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\ThemeManager",
    ));
    hkey.insert("ColorName".to_owned(), HkeyData::Delete);
    hkey.insert("DllName".to_owned(), HkeyData::Delete);
    hkey.insert("ThemeActive".to_owned(), "0".to_owned().into());

    hkey
}
