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

### Başlangıç Menüsü

### Maç Simülasyon Ekranı

### Güncel Fixtür Ekranı

### Transfer Market Ekranı

Transfer market ekranında transfer edilebilecek oyuncuların listesi yer alır. Oyuncuların id'si, tam ismi, oynadığı pozisyon, boy, transfer değeri ve istatistik ortalamalarına yer verilir.

```text
----------------------------------------------------------------
--------------- TRANSFER MARKET (73.91) Million $---------------
----------------------------------------------------------------
# 03-Riley Williams           Combo Guard 199.06cm 4.24 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	13.26    6.14     7.76     1.21     2.43     0.20    
# 08-Alex Wilson              Shooting Guard 207.63cm 6.24 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	17.65    9.29     1.42     0.99     1.25     3.70    
# 09-Morgan Martinez          Combo Guard 218.90cm 5.24 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	29.92    3.51     7.18     2.66     0.02     0.50    
# 11-Cameron Thomas           Small Forward 189.94cm 3.11 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	7.99     6.11     4.26     1.96     0.06     1.74    
# 15-Skyler Johnson           Small Forward 192.51cm 4.63 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	19.13    0.25     0.10     0.58     2.22     0.43    
# 20-Riley Brown              Small Forward 216.72cm 6.54 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	14.66    4.54     9.71     2.01     0.79     4.41    
# 21-Alex Miller              Small Forward 193.70cm 3.61 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	28.28    6.20     7.89     0.30     0.13     0.20    
# 26-Casey Williams           Shooting Guard 212.15cm 2.19 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	25.05    10.42    6.45     0.59     0.77     3.70    
# 29-Riley Johnson            Small Forward 194.39cm 2.72 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	6.93     6.88     7.32     2.18     1.49     4.25    
# 35-Jordan Smith             Combo Guard 218.28cm 5.75 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	18.65    3.69     5.31     2.01     1.20     2.82    
# 37-Jordan Wilson            Small Forward 207.99cm 9.39 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	22.97    5.60     6.22     1.78     2.95     3.40    
# 40-Quinn Jones              Small Forward 216.98cm 2.79 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	12.44    6.90     6.88     1.17     2.47     0.91    
# 44-Jordan Brown             Combo Guard 205.27cm 5.02 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	27.26    11.74    9.94     1.53     2.84     2.85    
# 46-Jamie Wilson             Shooting Guard 186.28cm 6.61 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	16.75    3.15     5.71     2.16     2.17     1.19    
# 48-Emerson Wilson           Power Forward 209.53cm 5.83 $
	Pnt      Reb      Ass      Blc      Ste      Trn     
	28.91    2.11     4.56     0.28     2.69     3.56    
----------------------------------------------------------------
```

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
