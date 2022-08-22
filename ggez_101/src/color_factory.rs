pub struct Color {
    pub name: String,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(name: String, red: u8, green: u8, blue: u8) -> Self {
        Self {
            name,
            red,
            green,
            blue,
        }
    }
}

pub fn get_colors() -> Vec<Color> {
    let mut colors = Vec::<Color>::new();

    colors.push(Color::new("Salmon Range".to_string(), 235, 106, 104));
    colors.push(Color::new("Red Lilac".to_string(), 109, 063, 091));
    colors.push(Color::new("Signal Yellow".to_string(), 229, 190, 001));
    colors.push(Color::new("Pastel Green".to_string(), 189, 236, 182));
    colors.push(Color::new("Light Pink".to_string(), 234, 137, 154));
    colors.push(Color::new("Signal Violet".to_string(), 144, 070, 132));
    colors.push(Color::new("Antique Pink".to_string(), 211, 110, 112));
    colors.push(Color::new("Telegrey 2".to_string(), 130, 137, 143));

    colors
}
