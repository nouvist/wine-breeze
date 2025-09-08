use std::collections::HashMap;

use crate::foundation::registry::{Hkey, HkeyData, HkeyPath};

pub fn create_classic_theme_registry() -> Hkey {
    Hkey {
        path: HkeyPath::current_user()
            .join(r"Software\Microsoft\Windows\CurrentVersion\ThemeManager"),
        content: HashMap::from([
            ("ColorName".to_owned(), HkeyData::Delete),
            ("DllName".to_owned(), HkeyData::Delete),
            ("ThemeActive".to_owned(), "0".to_owned().into()),
        ]),
    }
}
