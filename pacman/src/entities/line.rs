/*
   Map bileşeni oyun sahasını iki boyutlu bir matris olarak haritalandırmakta.
   Haritayı ekrana çizdirirken de bu vektörü matis gibi okuyabilmek lazım.
   Satır satır okumak bir yöntem. Buna göre satırlar bir veri yapısı olarak tutuluyor.

   Bunu bir slice(dilim) gibi düşünebiliriz. O yüzden Map türünden bir alan içermekte ve
   bu alan referans tabanlı olup belli bir yaşam döngüsüne tabi.
*/
use crate::common::contants::TILE_MAP_WIDTH;
use crate::entities::cell::Tile;
use crate::entities::map::Map;

pub struct Line<'a> {
    pub map: &'a Map,
    pub index: usize,
}

/*
   Map ile ifade edilen matris oyun sahasındaki hücreleri taşımakta.
   Matrisin her bir satırını Line ile ifade edebilirsek onun hücrelerinin dolaşmanın
   bir yolu da Iterator kullanmak.
*/
impl<'a> Iterator for Line<'a> {
    type Item = &'a [Tile];

    fn next(&mut self) -> Option<Self::Item> {
        let start_line_index = self.index * TILE_MAP_WIDTH;
        self.index += 1;
        if start_line_index >= self.map.tiles.len() {
            None
        } else {
            Some(&self.map.tiles[start_line_index..(start_line_index + TILE_MAP_WIDTH)])
        }
    }
}
