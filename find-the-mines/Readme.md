# Bevy Öğretisi

[Şu dev.to makalesini](https://dev.to/qongzi/bevy-minesweeper-part-1-534c) deniyorum. 

```shell
mkdir find-the-mines
cd find-the-mines

cargo init --bin . --name find-the-mines
cargo init --lib board-plugin
```

Özellikle find-the-mines projesindeki toml dosyasında board_plugin paketinin nasıl belirtildiğine dikkat edelim. Örnekte ayrıca bevy'nin çalışma zamanındaki önemli yardımcılarından inspector arabirimi de kullanılmakta. Böylece çalışma zamanında kameradan bord öğelerine, ark plandan arayüz bileşenlerine kadar birçok şeyi gözlemlemek mümkün. Debug etmeyi epeyce kolaylaştıran bir plugin olarak karşımıza çıkıyor.

```shell
# uygulamayı çalıştırmak için
cargo run
# World Inspector'un debugger'ı ile çalıştırmak için
cargo run --features debug
```

- Oyun tahtasında boş zeminler, mayın içeren zeminler, çevresinde belli sayıda mayın olan yerler var. Bu her oyun başladığında rastgele üretilecek bir tahta anlamına da geliyor. Oyun tahtasını tile_map veri yapısı ile ifade ediyoruz. Üstündeki bir karenin ne olduğu tile enum sabiti sayesinde bilinebilir. Tahtanın rastgele oluşturulması, bir hücrede mayın olup olunmadığını bilinmesi gibi fonksiyonlar tile_map tarafından karşılanmaktadır. İki boyutlu tahtanın üzerindeki hücreler için bir koordinat sistemine ihtiyaç vardır. Bu amaçla Coordinats isimli x,y değerlerini taşıyan bir veri yapısı kullanılmakta. Oyun tahtasının oluşturulması işini üstlenen enstrümanın bir plugin olarak app nesnesine eklenmesi için Plugin trait'ini implemente etmesi yeterlidir. Plugin işlevleri board_plugin paketindeki lib.rs altında yer almaktadır. 
