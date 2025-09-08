use std::collections::HashMap;

use crate::foundation::registry::{Hkey, HkeyPath};

pub fn create_dark_mode_registry(dark: bool) -> Hkey {
    let dword = match dark {
        true => 0,
        false => 1,
    };

    Hkey {
        path: HkeyPath::current_user()
            .join(r"Software\Microsoft\Windows")
            .join(r"CurrentVersion\Themes\Personalize"),
        content: HashMap::from([
            ("AppsUseLightTheme".to_owned(), dword.into()),
            ("SystemUsesLightTheme".to_owned(), dword.into()),
        ]),
    }
}
