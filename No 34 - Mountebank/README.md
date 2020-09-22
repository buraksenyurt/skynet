# Mountebank ile Mock Servis Desteği Sunmak

Mountebank, ne zamandır merak ettiğim ve denemek istediğim araçlardan birisiydi. Test senaryolarında kullanmak isteyeceğimiz mock servislerini kolayca inşa edebilmemize olanak sağlayan bir araç olarak tanımlayabilirim _(Şimdilik)_ Örneğin test kodumuz arka tarafta belki bir veritabanına bağlanan belki başka bir servis zincirini çağıran ya da farklı bağımlıkları olan bir servisi kullanmak zorunda olabilir. Normal şartlarda bu servisin ayakta olması zorunludur ki testimiz yürüsün. Ancak o anki test vakasının ilerleyen adımlarının işletilmesi için illaki bu servisin gerçekten de vereceği çıktıya ihtiyacımızı yoktur. Test vakası adımlarının devamı için o servisin vereceği çıktının sanki verilmiş gibi yapılarak ilerlenilmesi tercih edilen yöntemlerdendir. 

Üstelik kullandığı servisin hep aynı veri setini kullanarak çalışan bir testin, veri değişikliklerinden etkilenmemesi de istenebilir. Böyle durumlarda asıl servimiş gibi hareket eden _(Sahtekar/Taklitçi gibi isimlendirebiliriz bunları)_ ama testin ihtiyacı olup asıl vakayı bozmayacak şekilde kullanılabilen servisleri test senaryosu içerisine monte edebiliriz. Yani bir mock servis ile teste devam edelim diyebiliriz. 

İşte Mountebank, mock servislerin host edilmesi noktasında oldukça kullanışlı bir araç olarak karşımıza çıkıyor. Mountebank kendisi ile iletişim için bir REST API sunuyor. Bu API'yi kullanarak Mountebank'a mock servisler eklenebiliyor. Yani bir mock servis ihtiyacımız varsa bunu Mountebank'a yüklemek için HTTP Post çağrısı ile bir şeyler göndermemiz _(Stub'lardan oluşan Imposter aktörleri)_ gerekiyor. Mountebank'ın CI/CD hatlarına da entegre edilebildiği ifade ediliyor _(ki henüz gözümle görme şansım olmadı)_ İşte bu çalışmamın amacı Heimdall üstünde onu deneyimlemek ve nasıl çalıştığını, ne gibi bir çözüm sunduğunu anlayabilmek.

## Ön Hazırlıklar

Aşağıdaki adımları izleyerek devam edebiliriz.

```bash
mkdir asgard
cd asgard
npm init --yes
# Mountebank paketini npm aracı ile yüklüyoruz
# Birde yazacağımız mock servisleri Mountebank sunucusuna bildirmek için
# node-fetch paketinden yararlanacağız. Dolayısıyla onu da ekliyoruz.
npm i --save mountebank node-fetch

mkdir src
cd src
# port bilgilerini tutacağımız bir konfigurasyon dosyası ile 
# Bir Mountebank sunucusunu ayağa kaldırmaktan sorumlu index dosyasını oluşturuyoruz
# Bunlar src dizini altında konuşlanabilirler
touch ports.js index.js

# ve ilk Mock Service'imiz için aşağıdaki dosyayı kullanabiliriz
# yine src altında olabilir
touch ping-service.js

```

## Çalışma Zamanı

Uygulamanın çalışma zamanı için aşağıdaki adımları takip edebiliriz.

```bash
# Mountebank server'ını ayağa kaldırmak için asgard klasörü altında aşağıdaki komutu vermek yeterli
# bunun işletilmesi için package.json'a start komutunu ekledik.
# Normal o src klasörü altındaki index.js dosyasını çalıştırıyor
# Aşağıdaki komut sonrası sunucunun ayakta olup olmadığının sağlaması için
# http://localhost:5500 adresine gidebiliriz(Bir JSON içeriği görmemiz lazım)
npm start
```

_Mountebank server'ı npm start ile ayağa kaldırdığımızda 5500 portundan gelecek olan json içeriği_

![Screenshot_01.png](./assets/Screenshot_01.png)

Mock Servis örneklerini ekledikçe, imposter sözleşmelerinde belirtilen route tanımlarına giderek kayıt edilen servislerin çalışıp çalışmadığı kontrol edilmelidir. Örneğin ping-service'i Mountebank'a ekledikten sonra <http://localhost:5501/ping> adresine talete bulunup, stub'ın response kısmında belirtiğimiz pong cevabını almamız gerekir.

![Screenshot_02.png](./assets/Screenshot_02.png)

## Bomba Sorular

- Mountebank uygulamasına bir mock servis sözleşmesini _(imposter)_ NodeJs harici bir uygulamadan da _(Örneğin bir .Net Core)_ yollayabilir miyiz?

## Ödevler

- Eğer yukarıdaki bomba soruya cevabınız evetse, boşuna evet demiş olmayın. Bir örneğini yapın ;)