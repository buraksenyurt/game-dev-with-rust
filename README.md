# Hazır Oyun Motorlarını Kullanarak Rust Dili Yardımıyla Oyunlar Geliştirmek

Rust programlama dilinde oyun geliştirmek için popüler birkaç hazır çatıyı nasıl kullanacağımı öğrenmek istiyorum. 

Not! Tamamlayamadığım projeleri kendime ibretlik olsun diye garbage isimli klasöre bıraktım.

- [ ] **[33 - Fly](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/fly)]:** 2D platform oyun geliştirme ile ilgili denemeler yaptığım yer. Ana oyun motoru olarak bevy kullanıyorum ancak bu sefer fizik motoru için bevy_rapier' den yararlanıyorum. Rapier, RigidBody, Collider gibi hazır komponentler sağlıyor ve aynı zamanda çarpışma kontrolü, yer çekimi uygulanması, sürtünme kuvvetlerinin entegre edilmesi gibi daha birçok fizik motoru özelliğini destekliyor.
- [ ] **[32 - Shuttle Game](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/shuttle_game):** Bevy ECS sisteminin kullanıldığı bu örnekte [Marcel Champagne](https://github.com/marcelchampagne) öğretisinden yararlanarak ilerlemeye çalıştım. İlk kez 3D sistemde kamera kullanıyorum. Ağırlıklı olarka tüm yapı Plugin'ler üzerinden yürüyor. İncelemeye değer.
- [x] **[31 - Mathventure](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/mathventure):** Bu eğlenceli matematik oyununda farklı seviyeleri aşmak için çeşitli matematik sorularına doğru cevaplar vermemiz gerekiyor. Lunar-Landing ve Mono-Bird oyunlarında olduğu gibi yine SDL kütüphanesini kullandım. Var olan oyun motorları yerine state yönetimini kendi yapan bir motor yazmayı amaçladım.
- [x] **[30 - mono-bird](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/mono-bird):** Lunar Landing oyununda olduğu gibi SDL2 kütüphanesini kullandığım ve Flappy Bird klonunu yapmaya gayret ettiğim çalışmadır. Bu seferkinde bir oyun motorunun da temellerini atmaya çalışıyorum. En azından Game nesnesi ile ortam ayarlarını alan, pencereyi ayarlayan, fps'i belirleyen bir Engine nesnesi inşa etmeye çalışıyorum.
- [x] **[29 - lunar_landing](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/lunar_landing):** SDL2 kütüphanesini kullanarak geliştirmeye çalıştığım 1969 yapımı Lunar Landing klonu.
- [x] **[27 - on-my-way](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/on-my-way)** Bevy oyun motorunu kullanarak yazmaya çalıştığım bir uzay savaş oyunu. Kendi yolunda uzayı keşfe çıkmış kahraman pilotumuz karşısına çıkan meteorları yok etmeye çalışıyor. Füzeleri pahalı ve yakıttan yiyor ve yakıt sürekli azalıyor. Lakin yol boyunca insanlığın gezegenler arasına koyduğu otonomo yakıt istasyonları ile de karşılaşıyor. Onların üstünden geçerken belli süreliğine yakıtını tekrarda yenileyebiliyor. Ve macera bu şekilde gayet anlamsızca devam ediyor :D Maksat Rust kodlamak, Bevy ve ECS ile haşırneşir olmak.
- [x] **[26 - baller-game](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/baller-game)** Bu çalışmada Fredericj Joubert'in Bevy ECS sistemini örnek bir oyun yazarak anlattığı [video serisini](https://www.youtube.com/playlist?list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd) uyguluyorum.
- [x] **[25 - learning-bevy-ecs](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/learning-bevy-ecs)** Bevy ECS sistemindeki query kullanımını denediğim basit örnek.
- [x] **[24 - mini-quiz](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/mini-quiz)** Bu basit terminal oyununda kullanıcıya bir test havuzundan karışık sorular sorulmakta. Esasında Rust ile programlamaya giriş konusunda tasarladığım örneklerden birisi. Hem eğlenceli hem de bir şeyler öğretir türden.
- [x] **[23 - qa:](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/qa)** Yakın zamanda oyun programlama ile ilgili vereceğim bir seminerde, rust dilini tanıtmak için kullanacağım bir örneğe ihtiyacım vardı. Readme dosyasındaki maddeleri takip etmenizde yarar var. Klasik bir hello world'den ziyade dilin kendine has birkaç özelliğini vurgulamaya çalıştım.
- [x] **[22 - one-more-time: #bevy ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/one-more-time)** Daha önce bevy ile birkaç denemem olmuş ve hep hüsranla sonuçlanmıştı. Bu kez [şu adreste güzel bir youtube öğretisi](https://www.youtube.com/watch?v=VMDTEQC4vBI&t=9s) buldum. O seriyi takip edip Bevy oyun motorunu ve özellikle ECS (Entity Component System) yapısını anlamaya çalışıyorum. Tabi oyundan esinlenerek değişiklikler yapmaya da gayret ediyorum. Mesela kahramanımız hayali bir restoranda Donut'lar hazırlayıp satıyor. Birkaç çeşit donut var ve rastgele üretiliyorlar...
- [x] **[21 -radar: #ggez ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/radar)** Ggez küfesi kullanılarak geliştirmek istediğim bir terminal oyunu. Ekranda bir radar var ve arada sırada beliren nesneleri tıklayarak yok etmeye çalışacağız. Bazıları dost bazıları düşman kuvvetler. Dost kuvvetlere ateş edersek puanımız eksilecek düşman kuvvetleri vurursak puanımız artacak.
- [x] **[19 - mystery:](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/mystery)** Robert Kerr'ün [şu adresteki harika öğretisini](https://www.riskpeep.com/2022/08/make-text-adventure-game-rust-1.html) takip ederek metin tabanlı bir macera oyunu nasıl geliştirilir öğrenmeye çalışıyorum. Bu sırada Rust'ın temel bilgilerini de tekrar etmiş oluyorum. 
- [x] **[17 - pacman: #piston ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/pacman)** Klasik pacman oyununu [şu repodaki örneğine bakarak](https://github.com/mendess/rust-pacman) yazmaya ve rust kodlama bilgimi artırmaya çalıştığım örnek.
- [x] **[16 - pod-race: #rusty_engine ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/pod-race)** Ultimate Rust 2 kursundaki son örnek bir araba yarış oyunu. O bölümü tamamlamaya çalışıyorum.
- [x] **[15 - racer-ferris: #rusty_engine ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/racer-ferris)** Aldığım Ultimate Rust 2: Intermediate Concepts isimli Udemy eğitimde [rusty engine](https://cleancut.github.io/rusty_engine/00-welcome.html) kullanılarak geliştirilen bir oyun var. Bunu tamamlamaya çalışacağım.
- [x] **[12 - wing-pilot-2024: #macroquad ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/wing-pilot-2042)** Bu sefer Commodore 64'te sıkça oynadığım 1942 oyununun bir benzerini Macroquad çatısını kullanarak gerçekleştirmeye çalışıyorum.
- [x] **[11 - missile-commander: #macroquad ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/missile-commander)** Bu örnekte 80lerin meşhur Atari oyunlarından Missile Command'in bir benzerini yazmaya çalışıyorum. Amacım Missile Command arkasındaki matematiği keşfetmek ve Rust örneğinde uygulayabilmek. Ayrıca stage mantığını kurgulamaya ve seviyelerde ilerledikçe oyunu zorlaştırmaya çalışıyorum.
- [x] **[10 - life-game: #macroquad ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/missile-commander)** Conway'in life game oyununu Rust ile yazmaya çalıştım.
- [x] **[09 - Wolf-Tank: #macroquad ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/wolf-tank)** Macroquad motorunu kullanarak yazmaya çalıştığım basit bir RPG. Minik bir tankı kontrol etmeye çalışıyoruz.
- [x] **[08 - Tetra-Pong: #tetra ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/tetra-pong)** Basit 2D oyun geliştirme çatılarından olan [Tetra](https://tetra.seventeencups.net/) geliştirilen bir başka Ping-Pong örneği. Sitedeki tutorial'ı takip ediyorum.
- [x] **[07 - Spining-Square: #piston ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/spining-square)** Popüler oyun motorlarından olan [Piston](https://crates.io/crates/piston) ile ilgili Getting Started örneği.
- [x] **[05 - Math101:](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/dragon-fighter)** isimli projede oyun programlama için gerekli temel matematik enstrümanlara bakılıyor. [MathForGames](MathForGames.md) isimli dokümanda biraz daha fazla detay bulunabilir.
- [x] **[04 - walls_coming: #macroquad ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/walls_coming)** Sıradaki örnekte tekrardan Tantan'a uğruyorum. Bu kez Breakout oyununun yazılışını anlattığı [öğretiyi](https://youtu.be/xQ9YTY7ZgsI) çalışmaktayım. Bu örnekte [macroquad](https://github.com/not-fl3/macroquad) isimli başka bir oyun kütüphanesi kullanılmakta.
- [x] **[03 - rockstroid: #ggez ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/rockstroid)** isimli örnek aslında [Asteroids](https://en.wikipedia.org/wiki/Asteroids_(video_game)) türevli bir oyunun klonu. Ggez paketinin örnekleri arasında yer alan oyunu kodundan bakarak yazmaya çalışıyorum. Adım adım anlatımı yapılan bir öğreti olmadığından orta seviyede olsa Rust bilgisine ihtiyaç var. Building Block'ları keşfetmek açısından oldukça yararlı bir çalışma. Kod tarafında net olarak bir Entity Componenet System bulunmuyor ama ona epeyce yaklaşılmış.
- [x] **[02 - ggez_101: #ggez](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/ggez_101)** ggez küfesinin nasıl kullanıldığını anlamaya çalıştığım bu temel örnekte ekrana rastgele konumlarda ve farklı renklerde dikdörtgenler çizen bir kod parçası yer alıyor. Alt ok tuşuna basınca işleyiş duruyor üst ok tuşuna basınca rastgele konumlarda üretimler devam ediyor.
- [x] **[01 - ping_pong: #ggez ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/ping_pong)** İlk örnek ile paralel başladığım bu çalışmada ise severek takip ettiğim Youtuber'lardan birisi olan Tantan'ın Ping Pong oyununu yapmaya çalışıyorum. [Şu adreste](https://www.youtube.com/watch?v=TUE_HSgQiG0&list=PLY-17mI_rla7-lZ3Cj4mKLFXgEHaVGHWA) yer alan örnekte [GGEZ](https://crates.io/crates/ggez) crate kullanılıyor.
- [x] **[00 - crayz_invaders: #bevy ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/crayz-invaders)** isimli ilk örnekte Jeremy Chone'un [şu adresteki öğretisini](https://www.youtube.com/watch?v=j7qHwb7geIM) birebire takip ederek ilerlemekteyim. Bu örnekte [Bevy](https://crates.io/crates/bevy) isimli Crate kullanılıyor ve Space Invaders benzeri bir konsol oyunu geliştiriliyor. Bevy'nin kullanılması dışında _Entity Component System_ odaklı oyun motorlarının nasıl kullanıldığı da öğreniliyor.
- [ ] **[None - Garbage](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/garbage)** Tamamlayamadığım veya yarım kalan projelerin toplandığı yer.
  - [ ] **[06 - Dragon Fighter: #bevy ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/garbage/dragon-fighter)** isimli yeni öğretide Bevy küfesini kullanan bir oyun geliştirmeye çalışıyorum. Tekrardan Entity Component System _(ECS)_ konusunu irdeliyorum. Takip ettiğim örnek [şu github adresinde](https://github.com/mwbryant/rpg-bevy-tutorial/tree/master) yer almakta. Özellikle öğreti serisini branch olarak ayırması takibi kolaylaştırıyor. Tabii örnek bevy'nin 0.6 versiyonu baz alınarak hazırlanmış. Benim kullandığım versiyon ile arada ufak tefek farklılıklar olabiliyor. Bakalım sonuçta ortaya ne çıkacak :) **Youtube video anlatımı yavaşlatmama rağmen pek istediğim öğreticilikte değildir. Bu yüzden iptal ettim.**
  - [ ] **[14 - rpg-tutorial: #bevy](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/garbage/rpg-tutorial)** [Bu adreste](https://github.com/mwbryant/monster-fighter) yer alan örneği uygulayıp Bevy ile basit bir platform oyunu nasıl inşa edilebilir öğrenmeye çalışıyorum.~~ Valla bu ikinci denemem. Yine bitiremedim.
  - [ ] **13 - Minesweeper;** [Şu dev.to makalesindeki](https://dev.to/qongzi/bevy-minesweeper-part-1-534c) örneği takip ederek Windows'un meşhur mayın bulma oyununun Bevy oyun motoru ile nasıl yazıldığını öğrenmeye çalışıyorum.
  - [ ] **[20 - jumper: #bevy ](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/garbage/jumper)** Bevy kullanılan basit bir 2D platform macerası. Karakter sürekli zıplayarak bir borudan diğerine atlayarak ilerlemeye çalışır. Yere düşerse yanar. Ne kadar uzağa giderse o kadar başarılır olur. Arada sırada karışdan kendisine çarpabilecek kuş veya başka bir şey de gelebilir.
  - [ ] **[18 - mazes:](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/garbage/mazes)** Jamis Buck'ın [Mazes for Programmers](http://www.mazesforprogrammers.com/) isimli kitabının ilk bölümünde başlayan Binary Tree algoritmasına göre labirent oluşturma ve çözüm için Dijkstra algoritmasının uygulanmasını ele aldığım örnek. Kitap konuyu Ruby ile ele almış ben Rust'a evirmeye çalışıyorum.
  - [ ] **[28 - dungeon-day](https://github.com/buraksenyurt/game-dev-with-rust/tree/main/dungeon-day):** Bevy oyun motorunu kullanıp board oyunu yazdıran [çok güzel bir öğreti buldum](https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/). Hem detaylı hem de yazıya dökülmüş. Bevy'nin güncel sürümüne göre takip edip yazmaya çalışıyorum. Sonrasında buradaki bilgilerden faydalanıp yine yeni bir oyun yazmayı deneyeceğim.

## Örnek Çalışma Zamanları

Monochrome Bird oyununa ait çalışma zamanı görüntüsü aşağıdaki gibidir. 

![Monochrome-Bird](assets/mono-bird.gif)

1969 yapımı Lunar Landing oyunundan esinlenerek SLD2 kütüphanesini kullanarak yazmaya çalıştığım oyuna ait bir çalışma zamanı görüntüsü.

![Lunar Landing 2029](assets/lunar_landing.gif)

Bevy ile yazdığım On My Way isimli oyuna ait bir ekran görüntüsü. Oyunda Mars'a ulaşmak isteyen kahraman uzay gemimiz yoluna çıkan göktaşlarını yok etmeye uğraşıyor.

![assets/on-my-way.gif](assets/on-my-way.gif)

racer-ferris olarak adlandırdığım ama esasında boşluktaki bataryaları toplayan bir robotun yer aldığı örneğin çalışma zamanı. Wagner ile birlikte :D

![assets/eat_the_garbage.gif](assets/eat_the_garbage.gif)

WingMand 2042 oyununa ait bir ekran görüntüsü de aşağıdaki gibidir.

![assets/wingman2042.gif](assets/wingman2042.gif)

Atari Missile Command oyununa ait bir çalışma zamanı görüntüsü aşağıdaki gibidir.

![assets/miscmdgp.gif](assets/miscmdgp.gif)

Conway'in Game of Life oyununa ait bir çıktı aşağıdaki gibi oldu. 

![assets/game_of_life.gif](assets/game_of_life.gif)

Tank Wolf oyununa ait çalışma zamanı görüntüsü aşağıdaki gibidir. Bu ilk sürümde tankın kendi ekseni etrafında dönmesi, döndüğü yöne doğru ateşe edebilmesi, ileriye veya geriye doğru hareket edebilmesi, askeri birliklerin rastgele dizilerek ilkel bir AI mantığında tanka doğru hareket etmesi gibi özellikler yer almaktadır. Çarpışma hesaplamalarında birkaç bug var. Bazı koordinatlarda tank askeri birliğin altından geçmekte ve çarpışma hesabı suya düşmektedir ha haaa :D Bu tip şeyleri düzeltmek lazım ama Macroquad ile çalışmayı öğrenme noktasında bana epeyce deneyim kattığını söyleyebilirim. Diğer yandan AI tarafına iyice çalışmam lazım. Bunun için sanırım Pac Man'in arkasındaki AI matematiğini öğrenmem gerekiyor. Daha zamanım var.

![assets/wolf_tank.gif](assets/wolf_tank.gif)

Tetra framework kullanan öğretidekilerden yola çıkarak yazdığım tetra-pong isimli oyunun çalışma zamanı çıktısı da aşağıdaki gibi. GS ile FB karşı karşıya :P Aslında bir ping pong oyunu denemiştim ama farklı bir çatı kullanarak aradaki kolaylıkları kıyaslamaya çalıştım.

![assets/tetra-pong.gif](assets/tetrapong.gif)

Ping Pong örneğine ait çalışma sonuçlarım aşağıdaki gibi. Ekran kaydını mp4'ten gif formatına çevirdiğim için epey dandik ama az da olsa fikir veriyor :)
Güncellenen sürümde oyuncular için kendi taraflarına hareket eden birer iksir var. İksirleri yakaladıklarında ekstradan puanlar kazanıyorlar.

![assets/ping_pong.gif](assets/ping_pong.gif)

Rockstroid oyununa ait örnek ekran görüntüsü de aşağıdaki gibidir. Farklı bir örneğe geçeceğim için onu da yarım bıraktım :D Birçok bug içeriyor. 

- Söz gelimi bazı açılardaki lazer atışlarında lazer ters açıdan tekrar içeri girip belli bir mesafe kat ediyor. Oyuncu için hile kayalar için kötü :) 
- Lazer materyalinin açısı yanlış ve düzeltilmesi gerekiyor. 
- Uzay gemisine S tuşuna basınca yavaşlama fonksiyonelliği eklenebilir. Lineer interpolasyona göre yavaşlayabilir. Yani yavaşlama hızı artarak devam edip durur. 
- Kayalara hareket ettirilebilir. Çok yavaş da olsa gemiye doğru yönlenebilirler.

![assets/rockstroid.gif](assets/rockstroid.gif)
