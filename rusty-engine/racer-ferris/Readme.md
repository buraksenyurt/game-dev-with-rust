# Rusty_Engine ile Oyun Geliştirme Notları

Udemy'den [Nathan Stocks'ın](https://www.udemy.com/user/nathanstocks/) nefis bir Rust eğitimini almıştım. Son modülde kendi geliştirdiği Rust tabanlı oyun motoru [Rusty engine'e](https://github.com/CleanCut/rusty_engine/) ait bir örneği anlatmakta. Bende bu örneği tatbik ederek yeni bir şeyler öğrenmeye gayret ediyorum.

Oyun motorunun anlaşılması oldukça kolay. Özellikle sprite'ların kullanımı ve çarpışma tespitleri gayet anlaşılır ve basit. Ama dikkat edilmesi gereken hususlar var.

## Collision Detection ile İlgili Not

Oyun motorunu paket olarak kullanırken içerisinde hazır sprite'lar geliyor. Çarpışma sistemi için sprite'ları collision detection flag'lerini açmak yeterli ancak sprite'ların çaprışma çerçevelerini belirten collider uzantılı dosyalara ihtiyaç var. Bunun için de geliştirdiği bir uygulama var ve kolayca collider dosyasını üretebiliyoruz. Collider dosyasında aslında sprite'ın çerçevesinin çizildiği koordinat bilgileri yer alıyor.

```shell
cargo install rusty_engine --example collider

collider assets/sample.png
```

Uygulama basit bir grafik arayüze sahip ve mouse ile çerçeve noktalarını işaretleyip dir dosyaya bastırabiliyoruz. Terminale çıkan talimatları takip etmek lazım.

