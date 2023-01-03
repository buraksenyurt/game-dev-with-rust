/*
   Zeminde ne olduğu bilgisini tutan enum sabiti.
   Mayın olabilir, çevresinde mayınlar olan bir kare olabilir ya da boş olabilir
*/
#[cfg(feature = "debug")]
use colored::Colorize;

#[derive(Debug)]
pub enum Tile {
    Mine,
    MineNeighbor(u8),
    Empty,
}

impl Tile {
    // sabit bir fonksiyon bildirimi ile ilk kez karşılaştım.
    // Bu fonksiyon bir const gövdesi veya statik bir öğe tarafından çağırılabilir
    pub const fn is_mine(&self) -> bool {
        // matches makrosunu ilk kez kullandım
        // kendisinin Mine ile eşleşip eşleşmediğine kolayca bakabiliriz
        matches!(self, Self::Mine)
    }

    // Debug modda ekrana çıktı vermek için kullanabileceğimiz fonksiyon
    #[cfg(feature = "debug")]
    pub fn info(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Mine => "*".bright_red(),
                Tile::MineNeighbor(v) => match v {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => v.to_string().red(),
                },
                Tile::Empty => " ".normal(),
            }
        )
    }
}
