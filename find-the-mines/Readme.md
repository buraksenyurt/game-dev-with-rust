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
