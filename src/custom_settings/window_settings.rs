use iced::window::settings::Settings;
pub fn settings() -> Settings {
    let icon = iced::window::icon::from_file("src/custom_settings/window_settings.rs");
    if let Ok(icon) = icon {
        Settings {
            transparent: true,
            decorations: false,
            icon: Some(icon),
            ..Settings::default()
        }
    } else {
        Settings::default()
    }
}
