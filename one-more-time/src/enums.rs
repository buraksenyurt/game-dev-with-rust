use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum DonutType {
    Blue,
    White,
    Red,
}

impl Display for DonutType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let title = match self {
            DonutType::Blue => "Nefis orman meyveli tatlı çörek.",
            DonutType::White => "Vanilay ve saf sütten yapılmış kreması ile harika bir çörek.",
            DonutType::Red => "İçin alabildiğine vişne dolgusu ile kaplı şeytan çöreğimiz.",
        };
        write!(f, "{}", title)
    }
}

#[derive(PartialEq)]
pub enum Region {
    Upside,
    Center,
    Downside,
}
