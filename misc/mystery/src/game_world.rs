use crate::command::Command;
use crate::location::{Location, LOC_LAKEHOUSE, LOC_PLAYER, LOC_ROOM, LOC_SPACESHIP};
use crate::noun::Noun;
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
            player: Player {
                position: Location(0),
            },
            objects: vec![
                Object::new(
                    Noun::new("Oda".to_string(), "Odaya".to_string()),
                    "Maceranın başladığı yer.".to_string(),
                    None,
                ),
                Object::new(
                    Noun::new("Mekik".to_string(), "Mekiğe".to_string()),
                    "Kaptan köşkündesin.".to_string(),
                    None,
                ),
                Object::new(
                    Noun::new("Gölevi".to_string(), "Gölevine".to_string()),
                    "Çocukluğunda gittiğin göl evinin terasındasın.".to_string(),
                    None,
                ),
                Object::new(
                    Noun::new("Telsiz".to_string(), "Telsizi".to_string()),
                    "İletişim telsizini çalıştır.".to_string(),
                    Some(Location(LOC_ROOM)),
                ),
                Object::new(
                    Noun::new("Seyir Defteri".to_string(), "Seyir defterini".to_string()),
                    "Seyir defterindeki kayıtlara bak.".to_string(),
                    Some(Location(LOC_SPACESHIP)),
                ),
                Object::new(
                    Noun::new("Robot".to_string(), "Robotu".to_string()),
                    "Asistan robotunla konuş.".to_string(),
                    Some(Location(LOC_SPACESHIP)),
                ),
                Object::new(
                    Noun::new("Kitap".to_string(), "Kitabı".to_string()),
                    "'Denizler altında yirmi bin fersah'. Tekrar okumak ister misin?".to_string(),
                    Some(Location(LOC_LAKEHOUSE)),
                ),
                Object::new(
                    Noun::new("Köpek".to_string(), "Köpeği".to_string()),
                    "'Azman' ile oyna.".to_string(),
                    Some(Location(LOC_LAKEHOUSE)),
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
            Command::Quit => {
                "Simülasyon Kapatılıyor. Benimle oynadığın için teşekkürler :)".to_string()
            }
            Command::Unknown(input) => {
                format!(
                    "'{}'...Hım...Söylediğini nasıl yapabiliriz bilemedim.",
                    input
                )
            }
            Command::Talk(noun) => self.talk(noun),
            Command::Get(noun) => self.get(noun),
            Command::Give(noun) => self.give(noun),
            Command::Drop(noun) => self.drop(noun),
            Command::Inventory(noun) => self.inventory(noun),
        }
    }

    // Oyuncu bir takım eşyaları alabilir. Bunları alet çantasında tuttuğunu düşünelim.
    // Herhangibi andan 'araçlarımı listele' veya 'aletlerimi listele' yazarsa,
    // çantasında hangi nesneler olduğunu görebilir.
    pub fn inventory(&mut self, noun: &str) -> String {
        match noun {
            "araçlarımı" | "aletlerimi" => {
                let (tool_list, count) = self.get_objects(Location(LOC_PLAYER));
                if count == 0 {
                    "Alet çantanda hiçbir şey yok.\n".to_string()
                } else {
                    tool_list
                }
            }
            _ => {
                "Eğer aletleri listelemek istiyorsan 'araçlarımı listele' diyebilirsin.".to_string()
            }
        }
    }

    pub fn talk(&mut self, noun: &str) -> String {
        unimplemented!()
    }

    pub fn get(&mut self, noun: &str) -> String {
        unimplemented!()
    }

    pub fn give(&mut self, noun: &str) -> String {
        unimplemented!()
    }

    pub fn drop(&mut self, noun: &str) -> String {
        unimplemented!()
    }

    // Oyuncunun mekanlar arasında sıçramasını gerçekleştiren fonksiyon
    pub fn jump(&mut self, noun: &String) -> String {
        let (output_vis, obj_opt) = self.get_visible("nereye sıçrayacaksın?", noun);
        let player_loc = self.player.position;
        info!("Oyuncunun lokasyonu -> {}", player_loc.0);
        info!("Mekan lokasyonu -> {}", obj_opt.unwrap().0);
        match (obj_opt, player_loc) {
            (None, _) => output_vis,
            (Some(obj_loc), player_loc) if obj_loc.0 == player_loc.0 => {
                "Zaten oradasın.\n".to_string()
            }
            (Some(obj_loc), _) => {
                self.player.position.0 = obj_loc.0;
                info!("Oyuncunun geldiği lokasyon -> {}", self.player.position.0);
                "İşte...\n\n".to_string() + &self.look(&"etrafa".to_string())
            }
        }
    }

    // "Etrafa bak" için kullanılan fonksiyon
    pub fn look(&self, noun: &String) -> String {
        info!("look fonksiyonu çağırıldı. Noun bilgisi -> {}", noun);
        match noun.as_str() {
            "etrafa" | "" => {
                let (list_string, _) = self.get_objects(self.player.position);
                format!(
                    "Yer: {}\n{}\n",
                    self.objects[self.player.position.0].noun.name,
                    self.objects[self.player.position.0].description
                ) + list_string.as_str()
            }
            _ => "Nereye bakmak istediğini anlamadım.".to_string(),
        }
    }

    fn object_has_name(&self, object: &Object, noun: &String) -> bool {
        *noun == object.noun.real.to_lowercase()
    }
    fn get_object_idx(&self, noun: &String) -> Option<Location> {
        let mut result: Option<Location> = None;
        for (idx, obj) in self.objects.iter().enumerate() {
            if self.object_has_name(obj, noun) {
                result = Some(Location(idx));
                break;
            }
        }
        result
    }

    // Nesnelerin o an görünür olup olmadığını ve oyuncu tarafından kullanılıp
    // kullanılamayacağını tespit etmek için kullanılan fonksiyon.
    // Bakarak yazarken bile zorlandım ama özetle şu sorulara cevap veriyor...
    // - Nesne oyuncunun kendisi mi?
    // - Nesne oyuncunun bulunduğu lokasyonda mı?
    // - Nesne oyuncu tarafından zaten kullanılıyor mu?
    // vb
    fn get_visible(&self, message: &str, noun: &String) -> (String, Option<Location>) {
        let mut output = String::new();
        let input_loc = self.get_object_idx(noun);
        let real_loc = input_loc.and_then(|o| self.objects[o.0].location);
        let container_loc = input_loc
            .and_then(|o| self.objects[o.0].location)
            .and_then(|c| self.objects[c.0].location);
        let player_loc = self.player.position;

        match (input_loc, real_loc, container_loc, player_loc) {
            (None, _, _, _) => {
                output = format!("'{}'...Bunu anlayamadım.", message);
                (output, None)
            }
            (Some(object_idx), _, _, _) if object_idx.0 == player_loc.0 => {
                (output, Some(object_idx))
            }
            (Some(object_idx), _, _, player_loc) if object_idx.0 == player_loc.0 => {
                (output, Some(object_idx))
            }
            (Some(object_idx), Some(object_loc), _, _) if object_loc.0 == player_loc.0 => {
                (output, Some(object_idx))
            }
            (Some(object_idx), Some(object_loc), _, player_loc) if object_loc.0 == player_loc.0 => {
                (output, Some(object_idx))
            }
            (Some(object_idx), object_loc, _, _) if object_loc.is_none() => {
                (output, Some(object_idx))
            }
            (Some(object_idx), Some(_), Some(obj_container_loc), _)
                if obj_container_loc.0 == player_loc.0 =>
            {
                (output, Some(object_idx))
            }
            (Some(object_idx), Some(_), Some(obj_container_loc), player_loc)
                if obj_container_loc.0 == player_loc.0 =>
            {
                (output, Some(object_idx))
            }
            _ => {
                output = format!("Hiç bir '{}' görünmüyor burada\n", noun);
                (output, None)
            }
        }
    }

    pub fn get_objects(&self, location: Location) -> (String, i32) {
        info!("Mekandaki nesnelere bakılacak. Index -> {}", location.0);
        let mut output = String::new();
        let mut count: i32 = 0;
        for (idx, obj) in self.objects.iter().enumerate() {
            match (idx, obj.location) {
                (idx, _) if idx == 0 => continue,
                (_, None) => continue,
                (_, Some(obj_loc)) if obj_loc == location => {
                    if count == 0 {
                        output += "Seçeneklerin:\n";
                    }
                    count += 1;
                    output = output + &format!("\t- [{}], {}\n", obj.noun.name, obj.description);
                }
                _ => continue,
            }
        }
        (output, count)
    }
}
