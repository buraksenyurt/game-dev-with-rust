# Hazır Oyun Motorlarını Kullanarak Rust Dili Yardımıyla Oyunlar Geliştirmek

Rust programlama dilinde oyun geliştirmek için popüler birkaç hazır çatıyı nasıl kullanacağımı öğrenmek istiyorum. 

- [x] Crayz_Invaders isimli ilk örnekte Jeremy Chone'un [şu adresteki öğretisini](https://www.youtube.com/watch?v=j7qHwb7geIM) birebire takip ederek ilerlemekteyim. Bu örnekte [Bevy](https://crates.io/crates/bevy) isimli Crate kullanılıyor ve Space Invaders benzeri bir konsol oyunu geliştiriliyor. Bevy'nin kullanılması dışında _Entity Component System_ odaklı oyun motorlarının nasıl kullanıldığı da öğreniliyor.
- [x] İlk örnek ile paralel başladığım bu çalışmada ise severek takip ettiğim Youtuber'lardan birisi olan Tantan'ın Ping Pong oyununu yapmaya çalışıyorum. [Şu adreste](https://www.youtube.com/watch?v=TUE_HSgQiG0&list=PLY-17mI_rla7-lZ3Cj4mKLFXgEHaVGHWA) yer alan örnekte [GGEZ](https://crates.io/crates/ggez) crate kullanılıyor.
- [ ] ggez küfesinin nasıl kullanıldığını anlamaya çalıştığım bu temel örnekte ekrana rastgele konumlarda ve farklı renklerde dikdörtgenler çizen bir kod parçası yer alıyor. Alt ok tuşuna basınca işleyiş duruyor üst ok tuşuna basınca rastgele konumlarda üretimler devam ediyor.

Ping Pong örneğine ait çalışma sonuçlarım aşağıdaki gibi. Ekran kaydını mp4'ten gif formatına çevirdiğim için epey dandik ama az da olsa fikir veriyor :)
Güncellenen sürümde oyuncular için kendi taraflarına hareket eden birer iksir var. İksirleri yakaladıklarında ekstradan puanlar kazanıyorlar.

![assets/ping_pong.gif](assets/ping_pong.gif)