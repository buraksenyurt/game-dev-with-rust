use crate::command::Command;
use crate::location::Location;
use crate::player::Player;
use log::info;

// Oyun dünyasını temsil eden veri yapısıdır.
// Sahadaki oyuncuyu lokasyonu ile birlikte tutar.
// Ayrıca oyundaki mekanları da bir vektör dizisinde taşır.
pub struct GameWorld {
    pub player: Player,
    pub locations: Vec<Location>,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            player: Player { position: 0 },
            locations: vec![
                Location::new("Oda".to_string(), "karanlık bir odadasın.".to_string()),
                Location::new("Mekik".to_string(), "güvertedesin.".to_string()),
                Location::new(
                    "Göl Evi".to_string(),
                    "çocukluğunda gittiğin göl evinin terasındasın".to_string(),
                ),
            ],
        }
    }
}

impl GameWorld {
    // Oyuncunun girdiği komuta göre state güncellemesi için kullanılacak yardımcı fonksiyon
    pub fn update_state(&mut self, command: &Command) -> String {
        match command {
            Command::Jump(noun) => self.jump(noun),
            Command::Look(noun) => self.look(noun),
            Command::GetUp => "Ayaktasın.".to_string(),
            Command::Quit => "Kapatılıyor. Oynadığın için teşekkürler.".to_string(),
            Command::Unknown(input) => {
                format!("{} Söylediğini nasıl yapabiliriz bilemiyorum.", input)
            }
        }
    }

    // Oyuncunun mekanlar arasında sıçramasını gerçekleştiren fonksiyon
    pub fn jump(&mut self, noun: &String) -> String {
        let mut output = String::new();

        // var olan mekanlar dolaşılırken index değeri ve Location nesneleri ele alınır
        for (idx, location) in self.locations.iter().enumerate() {
            // eğer metin komutuyla gelen noun değeri kayıtlı bir location'a denk geliyorsa
            if *noun == location.name.to_lowercase() {
                // o anki lokasyonun indis değeri ile oyuncunun kendi pozisyon değerini karşılaştır
                if idx == self.player.position {
                    // Aynı ise zaten oradadır.
                    // Yani mekiğe gittiyse tekrar mekiğe gitmek istemesi halinde olduğu yerde kalmalıdır.
                    output = output + &format!("Zaten oradasın.\n");
                } else {
                    // Aksi durumda ise oyunucun pozisyonu yeni index değerine atanır
                    self.player.position = idx;
                    // Gidilen yerin neresi olduğunu söylemek "etrafa bak" ile çağırılan look fonksiyonu ile ele alınır
                    output = output + &format!("İşte...\n\n") + &self.look(&"etrafa".to_string());
                }
                break;
            }
        }

        if output.len() == 0 {
            format!("Nereye sıçramak istediğini anlayamadım.")
        } else {
            output
        }
    }

    // "Etrafa bak" için kullanılan fonksiyon
    pub fn look(&self, noun: &String) -> String {
        info!("look fonksiyonu çağırıldı. Noun bilgisi -> {}", noun);
        // eğer girilen gelime etrafa ise ya da hiçbir şeyse oyuncunun bulunduğu mekan bilgisi dönülür.
        match noun.as_str() {
            "etrafa" | "" => format!(
                "{}. Şu anda {}",
                self.locations[self.player.position].name,
                self.locations[self.player.position].description
            ),
            _ => format!("Nereye bakmak istediğini anlamadım."),
        }
    }
}
