// Oyun içindeki hareket eden nesneleri bir düşünelim.
// Oyuncunun kendisi, rastgele kayalar, oyuncunun ateş etmesi sonrası fırlayan lazer.
// Oyun jargonuna göre bunları Sprite olarak isimlendiriyoruz. Yer yer actor olarak da anılıyorlar.
// Aşağıdaki enum sabiti ile oyundaki aktörleri(ruhu olan hareketli nesneleri) ifade edebiliriz.
#[derive(Debug)]
pub enum SpriteType {
    Player,
    Rock,
    Shot,
}
