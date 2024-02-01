# Slam Dunk Manager Game Design Document

## Oyun Tanımı

- Adı  : Slam Dunk Manager
- Türü : Basketbol menajerlik
- Platform : Cross Platform Terminal

## Oyunun Amacı

- Ana Hedef : Oyunun ana amacı oyuncunun yönettiği basketbol takımını lig veya turnuva şampiyonu yapmasıdır.
- Oynanış Mekanikleri : Oyuncu sıfırdan takım kurar, oyuncu pazarından takımı için oyuncu transfer eder ve bu işlemleri takiben lig başlar. Maçlar simülasyon ile oynanır.

## Oyun Dünyası

- Mekan : Oyun günümüz dünyasında hayali takım ve oyuncu isimlerinden oluşan 8 takımlı bir lig olarak kurgulanır.
- Kurallar : Oyunun temel kuralları aşağıdaki gibidir.
  - Oyuncu oyuna başladığında lig sonlanana ya da ligi sıfırlayana kadar sadece tek bir takım oluşturabilir.
  - Oyuncunun takımı kurması için başlangıçta belli bir bütçesi vardır. Ligler tamamlandıkça ve oyun sıfırlanmadığı sürece bütçe azalıp çoğalabilir.
  - Oyun transferleri transfer marketten yapılır.
  - Oyuncu takımını kurguladığında maksimum 15 oyuncu transfer edebilir.
  - Transfer serbestliği lig boyunca vardır. Ligin herhangi bir zamanında oyuncu satılabilir veya alınabilir.

## Oyun Mekanikleri

- Takım Yönetimi :
- Bütçe Yönetme :
- Maç simülasyonu : 

## Karakterler

- Oyuncu Profili :
- Takımlar ve Özellikleri :

## Görsel ve Ses Tasarımı

- Arayüz Tasarımları :
- Ses ve Müzik :

## Kullanıcı Arayüzü ve Deneyimi

- Menü ve Ekran Tasarımları :
- Kullanıcı etkileşimi :

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
