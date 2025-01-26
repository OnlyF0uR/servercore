use pumpkin_util::text::{color::RGBColor, TextComponent};

pub fn error_colour() -> RGBColor {
    RGBColor::new(255, 46, 105)
}

pub fn neutral_colour() -> RGBColor {
    RGBColor::new(105, 200, 255)
}

pub fn success_colour() -> RGBColor {
    RGBColor::new(105, 255, 145)
}

pub fn todo_message() -> TextComponent {
    TextComponent::text("Function not yet implemented").color_rgb(error_colour())
}
