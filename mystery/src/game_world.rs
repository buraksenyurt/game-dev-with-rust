use crate::command::Command;
use crate::location::Location;
use crate::object::Object;
use crate::player::Player;
use log::info;

// Oyun dünyasını temsil eden veri yapısıdır.
// Sahadaki oyuncuyu lokasyonu ile birlikte tutar.
// Oyundaki yerler, aletler ve aktörler de object türünden bir vektörde tutulur.
pub struct GameWorld {
    pub player: Player,
    pub objects: Vec<Object>,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            player: Player { position: 0 },
            objects: vec![
                Object::new("Oda".to_string(), "karanlık bir odadasın".to_string(), None),
                Object::new("Mekik".to_string(), "güvertedesin".to_string(), None),
                Object::new(
                    "Gölevi".to_string(),
                    "çocukluğunda gittiğin göl evinin terasındasın".to_string(),
                    None,
                ),
                Object::new(
                    "Telsiz".to_string(),
                    "iletişim telsizi".to_string(),
                    Some(Location(0)),
                ),
                Object::new(
                    "Seyir Defteri".to_string(),
                    "seyir defteri".to_string(),
                    Some(Location(1)),
                ),
                Object::new(
                    "Robot".to_string(),
                    "asistan robot".to_string(),
                    Some(Location(2)),
                ),
                Object::new(
                    "Kitap".to_string(),
                    "denizler altında yirmi bin fersah".to_string(),
                    Some(Location(3)),
                ),
                Object::new("Köpek".to_string(), "azman".to_string(), Some(Location(4))),
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
        for (idx, obj) in self.objects.iter().enumerate() {
            // eğer metin komutuyla gelen noun değeri kayıtlı bir location'a denk geliyorsa
            if *noun == obj.name.to_lowercase() {
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
                self.objects[self.player.position].name,
                self.objects[self.player.position].description
            ),
            _ => format!("Nereye bakmak istediğini anlamadım."),
        }
    }
}
