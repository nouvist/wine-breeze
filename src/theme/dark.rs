use crate::foundation::registry::{Hkey, HkeyPath};

pub fn create_dark_mode_registry(dark: bool) -> Hkey {
    let mut hkey = Hkey::new(HkeyPath::new(
        r"HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
    ));

    let dword = match dark {
        true => 0,
        false => 1,
    };

    hkey.insert("AppsUseLightTheme".to_owned(), dword.into());
    hkey.insert("SystemUsesLightTheme".to_owned(), dword.into());

    hkey
}
