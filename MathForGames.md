# Oyun Geliştirmede Kullanılan Temel Matematik Enstrümanlar

Oyun motorları arasında popüler olan Unity, Unreal gibi pek çok platform fizik motorlarından, ışıklandırmaya, gölgeleme efektlerinde çarpışma hesaplamalarına kadar pek çok şeyi hazır olarak sunar. Ancak oyun geliştirme üzerine programlamanın temellerinde her zaman olduğu gibi matematik vardır. Aşağıda temel seviyede oyun geliştirme için bilinmesi gereken matematik enstrümanlarına yer verilmekte.

## Pisagor Teoremi

Oyuncunun roketi iki boyutlu sahada ilerlerken etrafını saran düşman gemileri rast gele yönlerde hareket ediyordu. Bazı düşman gemileri belli mesafe kadar yaklaşınca da oyuncunun gemisine ateş etmekteydi. İşte soru; düşman gemisinin ateş etmek için oyuncuya ne kadar yaklaştığını nasıl buluruz? Problemi basitleştirmek adına oyunun iki boyutlu bir sahada yazıldığını düşünelim. Oyunlarda koordinat düzleminin ekranın sol üst köşesini 0,0 başlangıç noktası olarak kabul ettiğini unutmayalım.

![assets/math_101_01.png](assets/math_101_01.png)

Pisagor teoremini math101 isimli rust projesinde deniyoruz. Aşağıda gördülüğü gibi daire, kareye 50 pixel altında yaklaştığında ekrana bir uyarı mesajı geliyor.

![assets/pisagor.gif](assets/pisagor.gif)

## Vektörler ve Açılar

Oyunlarda vektörler sıklıkla kullanılır. Bir vektör ile büyüklük _(daha çok uzunluk olarak da ifade edilir)_ ve en önemlisi yön bilgileri ifade edilebilir. Bu sayede oyun sahasındaki bir nesnenin hareket yönü vektörler ile ifade edilebilir. Büyüklük _(Magnitude)_ yerine yönün _(direction)_ önemli olduğu durumlarda genellikle birim vektörler değerlendirilir. Bir vektörün uzunluğunu _(büyüklüğünü)_ bulmak için pisagor teoremi kullanılabilir. Nitekim uzaydaki bir vektörün uzunluğu iki nokta arasındaki mesafeye tekabül eder ki bu da pisagor üçgeninden yararlanılarak hesap edilir.

![assets/math_101_02.png](assets/math_101_02.png)

Vektörler sayesinde bir noktanın merkeze olan uzaklığını ifade etmek de kolaydır. Örneğin görseldeki araba her yön değiştirdiğin gitti mesafeyi büyüklük olarak kabul eden ve bir yönü olan vektörler ile matematiksel olarak ifade edilebilir. Buna göre aracın son geldiği Ex,Ey noktasının vektörel formdaki karşılığını bulmak kolaydır. Ayrıca Ex,Ey noktasının vektörel ifadesi arabanın merkezden _(merkez olarak başka bir nesne konumu örneğin yol kenarındaki bariyer de kabul edilebilir)_ ne kadar uzakta olduğunu söyler. Lakin az önce belirttiğimiz üzere özellikle iki boyutlu saha kullanan oyunlarda vektörün büyüklüğü _(uzunluğu)_ göz ardı edilebilir. Yön _(direction)_ daha önemlidir. Bu sebeple birim vektöre _(Unit Vector)_ sıklıkla rastlanır. Herhangi bir vektörü normalleştirme _(normalizing)_ işlemine tabi tutarak birim vektör cinsinden ifade etmek mümkündür.

Vektörlerde 0,0 konumuna göre kurulan dik üçgenlerden yararlanılarak yönü belirten açılar hesaplanabilir. Genelde bu hesaplamalar dik üçgenin karşıt kenarı ile komşu kenar arasındaki oranın arktanjantı şeklinde hesaplanır _(tanjantının -1 üssüdür esasında)_ ve derece cinsinden bulunur. Oyun motorlarının çoğunda derece yerine radyan kullanılır. Bir daireyi dört eşit dilim olarak böldüğümüzde radyan ile dilimlerin pi değeri cinsinden ifade edilebilmesi sağlanır. Bulunan açının radyana çevrilmesi ya da tam tersinin yapılması da formüller ile mümkündür.

Yön için önem arz eden açının bulunmasında sadece tanjant değil zaman zaman sinüs ve kosinüs fonksiyonları da kullanılabilir. 

- Elimizde açının karşı kenar uzunluğu ile hipotenüs değeri varsa sinüs fonksiyonundan yararlanılarak açı bulunabilir.
- Elimizde bulunmak istenen açının karşıt kenar uzunluğu ile komşu kenar uzunluğu varsa tanjant fonksiyonundan yararlanılır.
- Son olarak elimizde, bulunmak istenen açının komşu kenar uzunluğu ile hipotenüs değeri varsa kosinüs fonksiyonundan yararlanılabilir.

Bu noktada açı ile vektör arasındaki ilişkiyi ve dolayısıyla bir vektörün yönünü bulmayı iyi anlamak gerekir. Elimizde bir açı varsa birim vektör cinsinden yönü bulmak oldukça kolaydır. Buna göre açının sinüsü y değerini, kosinüsü de x değerini bulmamızı sağlar.

Sinüs eğrisinin kullanıldığı örnek kod parçasının çalışması aşağıdaki gibidir. Sol ve sağ ok tuşlarına basıldığında dairenin x kooridanatı değerine göre sinüs değeri hesaplanır ve y değeri buna göre değiştirilir. Sonuçta altın renkli topun sinüs eğrisine göre hareketi söz konusudur. Tabi normalde sinüs eğrisine baktığımızda ilk hareketin yukarı yönlü başladığını görürüz. Ancak burada koordinat sisteminde 0,0 orjininin ekranın sol üst köşesinde olduğunu hatırlayalım. Yine de y değerinin artım ve azaltımını duruma göre değiştirip aşağı ve yukarı yönlü hareketleri kontrol edebileceğimizi unutmayalım.

![assets/sinecurve.gif](assets/sinecurve.gif)

## Nokta Çarpım (Dot Product)

İki boyutlu oyun sahasında nesnelerin yönleri arasındaki açının değerlendirildiği pek çok durum var. Örneğin bir uçağın bir checkpoint noktasında geçip geçmediğini anlarken ya da yokuş aşağıya indiğini veya bir yokuşu çıktığını hesaplarken açıları kullanabiliriz. Aşağıdaki gösterimde uçağın yönü ile checkpoint noktasının yönü arasındaki ilişkinin nokta çarpım ile ele alınışı değerlendirilmekte. Bu senaryolarda birim vektörlerin göz önüne alındığını baştan belirteyim. Nitekim vektörün büyüklüğünden ziyade yönü önemli. Birim vektör cinsinden bir nesnenin yönünü ifade ettiğimizde aradaki açıyı bulmak için nokta çarpımı formülasyonundan da yararlanabiliyor. Nokta çarpım hesaplaması ters tanjant ya da ters kosinüs ile bulunan açı hesaplamasına göre işlemciye daha az yük bindiriyor. Yani işlem maliyeti çok daha ucuz. 

![assets/dotproduct1.png](assets/dotproduct1.png)

Nokta çarpımını birim vektöre indirgediğimizde elde edilen skalar değer -1 ile 1 aralığında olur. Buna göre vektörlerin aynı veya ters yönde olduklarını ya da birbirlerine yaklaştıklarını veya uzaklaştıklarını anlayabiliriz.

![assets/dotproduct2.png](assets/dotproduct2.png)

## Doğrusal İnterpolasyon (Linear Interpolation)

Oyun sahasında başvurulan matematik enstrümanlarından birisi de Linear Interpolation kavramıdır. Örneğin motion efektlerinde, belli bir rotayı izlemesi istenen unsurlarda, renk geçişlerinde, giderek hızlanan veya yavaşlayan nesnelerde sıklıkla başvurulur. En basit formu da iki nokta arasında gidilmesi istenen mesafenin sürece kesin olduğu motion efektleridir.

Şöyle düşünelim. Diyelim ki x,y değerlerini bildiğimiz A ve B noktaları var. Düşman gemisinin A noktasından B noktasına 10 saniyede gideceğini de biliyoruz. Amacımız herhangi bir t anında bu nesnenin iki nokta arasındaki doğruda hangi x,y koordinatlarında olduğunu öğrenmek ki buna göre onun otomatik hareketlenmesini sağlayacağız. Aşağıdaki grafikte bu hesaplama için kullanılabilecek formüle yer veriliyor.

![assets/linear_inter_01.png](assets/linear_inter_01.png)

Başlangıç aşamasında A konumunda olan nesnenin 10ncu saniyede B konumunda olacağını biliyoruz. Buna göre örneğin C konumundan geçerken ki x,y koordinatlarını bulmak istersek nasıl bir formül kullanabiliriz? Esasında zaman çizelgesini yüzdesel olarak ifade edersek işimiz biraz daha kolaylaşır. Başlangıç konumu olasılığın %0 hali ile ifade edilirse 10ncu saniyede varış noktasına gelmiş olmamız da %100'e karşılık gelir. Yani varış noktasında isek nesne yüzde yüz kesinliklte rotasını tamamlamıştır. Bir başka deyişle başlangıç ve bitiş noktalarını yüzdesel olarak düşündüğümüzde 0 ile 1 arasında yer alan bir olasılık değerinden bahsedebiliriz. Bu değer formülümüzdeki t parametresine karşılık gelir ve oyunda kullanılan FPS(frame per second) bilgisine göre de ayarlanabilir. Örneğin her bir frame 0.1 saniyede geçiliyorsa bir T anını T=T+0.1 gibi ifade edebiliriz. Kendi senaryomuzda bu T değerinin 10'a bölümü hesaplamadaki t değerini verecektir. Bir başka deyişle elimizde FPS değeri de varsa herhangi bir andaki t değerini hesaplamak kolaydır.

İşin zorlaştığı noktalardan bir tanesi yerçekiminin devreye girdiği ya da rotanın doğrusal olarak ifade edilemediği eğrilerden oluşan senaryolardır. Örneğin önündeki tepeden space tuşuna basınca sıçyarak karşı tepeye ulaşmaya çalışan bir arabayı göz önüne alalım. Böyle bir durumda yerçekimine göre bir eğri çizilmesi ve bunun zaman bağımlı olarak hesaplanması gerekir. Bu sefer aşağıdaki şekli göz önüne alalım. Hareket halindeki aracın bulunduğu noktayı bir vektör olarak ifade edeceğiz _(P ile ifade edilen kısım)_. Ayrıca t anında gittiği yönü taşıyan birde hız vektörü kullanmaktayız _(V ile ifade edilen)_. t ile oyun motorlarının bize genellikle hazır olarak verdiği delta time değerini ifade ediyoruz.

![assets/linear_inter_02.png](assets/linear_inter_02.png)

Aracın t+1 anındaki konumunu ifade eden vektörü bulmak için hız vektörü ile delta time bilgisinin yer aldığı bir formül kullanılır. Ancak öncesinde o anki hız değerini işaret eden vektörün yine delta time ve yer çekimi vektörünün hesaba katılacağı bir formül ile bulunması gerekir. A ile ifade edilen _(genelde Acceleration olarak bilinir)_ vektör bu senaryoda yerçekimini işaret eden sabit bir vektör değeridir.

Elbette oyun geliştirmede kullanılan matematiksel enstrümanlar bunlarla sınırlı değil. Matrisler, lineer cebir gibi daha bir çok konu başlığı bulunmakta. İlerleyen zamanlarda bu konulara da değinmek düşüncesindeyim.