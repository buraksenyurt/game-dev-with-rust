# Slam Dunk Manager Game Design Document

## Oyun Tanımı

- Adı  : Slam Dunk Manager
- Türü : Basketbol Menajerlik
- Platform : Cross Platform Terminal

## Oyunun Amacı

- Ana Hedef : Oyunun ana amacı oyuncunun yönettiği basketbol takımını lig veya turnuva şampiyonu yapmasıdır.
- Oynanış Mekanikleri : Oyuncu sıfırdan takım kurar, oyuncu pazarından takımı için oyuncu transfer eder ve bu işlemleri takiben lig başlar. Maçlar simülasyon ile oynanır.

## Oyun Dünyası

- Mekan : Oyun günümüz dünyasında hayali takım ve oyuncu isimlerinden oluşan 8 takımlı bir lig olarak kurgulanır.
- Kurallar : Oyunun temel kuralları aşağıdaki gibidir.
  - Lig 3X3 turnavalar içerecek şekilde tasarlanır. 
  - Her takımın en fazla 5 oyuncusu olabilir.
  - Takımların herbirinin lig başlangıcındaki savunma ve hücum güçleri eşittir. Lig oynandıkça skor ve başka parametrelere göre bu değerler düşer veya artar.
  - Oyuncu oyuna başladığında lig sonlanana ya da ligi sıfırlayana kadar sadece tek bir takım oluşturabilir.
  - Oyuncunun takımı kurması için başlangıçta belli bir bütçesi vardır. Ligler tamamlandıkça ve oyun sıfırlanmadığı sürece bütçe azalıp çoğalabilir.
  - Oyun transferleri transfer marketten yapılır.
  - Oyuncu takımını kurguladığında maksimum 5 oyuncu transfer edebilir.
  - Transfer serbestliği lig boyunca vardır. Ligin herhangi bir zamanında oyuncu satılabilir veya alınabilir.

## Oyun Mekanikleri

- Takım Yönetimi : 8 takımlı bir lig başlatıldığında 7 takım rastgele oyunculara sahip olaraktan otomatikman oluşturulur. 8nci takım koçun vereceği isme bağlı olarak transfer marketten bütçeye göre alınan oyunculardan oluşturulur.
- Bütçe Yönetme : Koçun lig başlangıcında kullanabileceği belli bir bütçe vardır. Lig başlarken oyuncu havuzundan rastgele oyuncular rastgele 7 takıma dağılır. Kalan oyuncular transfer markete alınır. Takım koçu bu oyuncu havuzundan bütçesine göre transfer yapar ve kendi takımını kurar. Lig boyunca takım bütçesi ve transfer marketteki havuza göre oyuncu transferleri yapılabilir.
- Maç simülasyonu : Maçlar terminal ekranında 1 dakika içinde oynanacak şekilde simüle edilir. Maç başlarken oyuncu ilk 3 oyuncusunu seçebilir veya bu atamayı programa bırakabilir. Maç oynanırken skor bilgisi ile ilgili bilgilendirme yapılır. Tek pota 3X3 turnuvalar olduğundan 1 dakika gerçek hayattaki 8 dakikaymış gibi kurgulanır. Oyun sırasında sakatlık olması halinde veya takım oyuncusunun enerjisi belli bir değerin altına indiğinde oyuncuya değişiklik isteyip istemediği sorulur ve oyuncu değişitirme hakkı da tanınır. 

## Karakterler

- Oyuncu Profili : Oyuncular guard, shooter guard, short forward, power forward, pivot gibi posiyonlara sahiptir. Enerji seviyeleri ve bazı temel istatistik ortalamaları mevcuttur. Enerji seviyeleri ilk başlangıçta aynı olup oyun simülasyonu sırasında değişiklik gösterir. Örneğin pozisyon, boy ve oyunda kalma süresine göre giderek azalır. Belli bir seviyenin altına inildiğinde oyuncu değişikliği için koça bilgi verilebilir.
- Takımlar ve Özellikleri : Ligde 7 + oyuncunun takımı olmak üzere 8 takım vardır. Takımlar 5er kişilik kadrolardan oluşur. Maçlara 5 kişi olarak çıkılır ve maçı 3 kişi yapar. Her takımın hücum ve savunma güçleri vardır. Bunlar ligin ilerleyişine göre belirli koşullar altında azabilir veya artabilir.

## Görsel ve Ses Tasarımı

- Arayüz Tasarımları :
- Ses ve Müzik :

## Kullanıcı Arayüzü ve Deneyimi

- Menü ve Ekran Tasarımları :
  - Başlangıç Menüsü :
  - Maç simülasyon ekranı :
  - Güncel fixtür ekranı : 
- Kullanıcı etkileşimi : Oyuncu terminal gelen oyun isteklerine klaveyeden yaptığı girdilerle cevap verir. Menü seçeneklerindeki sayısal değerlere basarak menüler arası geçiş yapar.

## Geliştirme Takvimi

- Geliştirme süreci aşamaları :
- Milestones ve hedef tarihler :

## Teknik Detay

- Teknolojiler : Oyun Rust programlama dili ve minimum seviyede üçüncü parti küfe kullanılarak geliştirilecektir. Tamamen terminalden metin tabanlı çalışacak şekilde tasarlanacaktır. Geliştirme ortamı olarak Rust Rover IDE'sinden yararlanılmaktadır. Oyun Ubuntu 22.04 platformu üstünde geliştirilmektedir.  
- Sistem Gereksinimleri : Terminalden çalışan text tabanlı bir oyun olduğundan minimum sistem gereksinimiyle çalışabilecek şekilde tasarlanmaktadır. Ancak minimum gereksinim için şu konfigurasyon verilebilir.

| Özellik | Minimum Gereksinim            |
|---------|-------------------------------|
| OS      | Ubuntu 16.04 LTS veya üzeri   |
| CPU     | 1 GHz veya daha hızlı işlemci |
| RAM     | 512 MB veya daha fazla        |
| Disk    | Minimum 25 Mb boş alan        |
