# Slam Dunk Manager

Terminalden çalışan bir basketbol menejerlik oyunu yazmaya çalışacağım.

## Proje Yapısı

Projenin genel yapısı ile ilgili bilgiler burada yer alabilir. Örneğin modül ve klasör yapıları ile oyun mekanikleri, veri yapıları ve kullanıcı arayüzünü bu kısımda detaylandırıp anlatabiliriz.

```text
src/
|--- main.rs
|--- lib.rs     (Projenin ana yapı taşlarını ait tanımlamalar)
|--- game/      (Oyun mekanikleri ve mantığını içeren modüller)
|       |--- mod.rs 
|       |--- team.rs
|       |--- player.rs
|       |--- match.rs
|--- ui/        (Kullanıcı arayüzü ile ilgili işlevsellikler)
|       |--- mod.rs
|       |--- terminal.rs
|--- util/      (Yardımcı fonksiyonlar ve genel amaçlı modüller)
|       |--- mod.rs
|       |--- random.rs
|--- data/      (Serileştirilebilir veri yapıları, veri yükleme ve kaydetme operasyonları)
        |--- mod.rs
        |--- model.rs
```

### Oyun Mekanikleri

Oyunun nasıl işleyeceği, hangi özellikleri içereceği ve kullanıcı etkileşiminin nasıl yapılacağı ile ilgili bilgileri buraya ekleyebiliriz.

- Takım Seçimi :
- Maç Simülasyonu :
- Oyuncu Transferleri :
- Transfer market borsası :

### Veri Yapıları

Oyundaki veri modellerini buraya ekleyebiliriz.

- Takımlar :
- Oyuncular :
- Maç Sonuçları :

### Kullanıcı Arayüzü

Terminalden kullanıcıya bilgiyi nasıl sunabiliriz, kullanıcıdan bilgileri nasıl alabiliriz, ekranlar nasıl olabilir.

## Üçüncü Parti Kütüphaneler

- Verileri fiziki olarak saklarken JSON formatından yararlanılabilir. serde küfesini kullanabiliriz.
- Rastgele olayların icrası (maç sonuçları, sakatlık vb) için rand küfesini kullanabiliriz.

## Oyun Döngüsü

- Oyun başlangıcı, takım seçimi, maç simülasyonu , sonuçların gösterimi gibi adımları içeren ana döngünün tasarlanması.
- Kullanıcıdan girdi alma, girdiye göre hesaplamaların yapılması ve sonuçların çıkartılması ile ilgili bir oyun döngüsünün tasarlanması veya üstte bahsedilen döngüye eklenmesi.

## Oyun Mekaniklerinin Geliştirilmesi

- Takımlar ve oyuncular için güç, hız, savunma, taktik vb özelliklerin detayları. 
- Maç simülasyonu sırasında gerçekleşecek fauller, teknik fauller, sakatlıklar, basketler vs gibi rastgelelik içeren özelliklere ait detaylar.

## Kullanıcı Etkileşimi

- Kullanıcıya verilecek talimatların net ve anlaşlılır olarak belirlenmesi.
- Maç sonuçlarını, sıralama tablosunu, önemli olayları anlatan metinlerin hazırlanması.