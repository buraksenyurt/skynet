# skynet

Kodlama tarafında pas tutmamak ve iyi vakit geçirmek adına internetten bulup çalıştığım bazı konuların örnek kodlarını burada toplamaya çalışıyorum. README.md dosyaları içerisinde amacımı ve az biraz proje başlangıç adımlarını da tarifliyorum. Bir sene sonra aynı örneğe tekrar dönüp baktığımda kodlar tanıdık geliyor ve anlıyorsam çalışmam boşa gitmemiş demektir. Ve sen sevgili yazılım sevdalısı arkadaşım; giriş yazılarından faydalanıp proje açılışlarını yapabiliyorsan, kodları kendi ortamında çalıştırabiliyorsan, sağıyla soluyla oynayarak bozup düzeltiyor hayal gücünle bir şeyler ekliyorsan, senin için de işe yaramış demektir. _(Asla bir [Saturday Night Works olamayacak](https://github.com/buraksenyurt/saturday-night-works) ama olsun.)_

## Konular

~~Burada Ahch-To _(MacOS Mojave - Intel Core i5 1.4Ghz, 4 Gb 1600 Mhz DDR3)_ üzerinde geliştirmeye çalıştığım kişiel öğretilerim yer almaktadır.~~ Örnekleri bir süredir Heimdall isimli Ubuntu yüklü makinemde geliştiriyorum. Ancak ara sıra şirket bilgisayarındaki Windows'u da kullandığım oluyor. Ağırlık Heimdall tarafında. _(Heimdal = Ubuntu, Ahch-To = MacOs olarak düşünülebilir. T-Shirt Size ile projelerin kendime göre zorluk derecelerini belirtiyorum. Örneğin XL, o örnekte çok zorlandığım anlamına geliyor)_

| No | Konu                                                                                                         | İlerleme           | Seviyesi |      İlk Ortamı     | Durum |
|----|--------------------------------------------------------------------------------------------------------------|--------------------|:--------:|:-------------------:|------:|
| 01 | Ruby Tarafından Redis _(Docker bazlı)_  Veritabanı ile Konuşmak                                              | _(=========->%95)_ |     M    |       Ahch-To       | Pasif |
| 02 | MongoDB ile bir GO uygulamasını konuşturmak                                                                  | _(========->%85)_  |     L    |       Ahch-To       | Pasif |
| 03 | Razor Öğreniyorum                                                                                            | _(========>%80)_   |     M    |      Windows 10     | Pasif |
| 04 | IKU Meetup _(MongoDB, Web API, Unit Test)_                                                                   | _(Tamamlandı)_     |     L    |      Windows 10     | Pasif |
| 05 | gRPC Tabanlı Basit Bir .Net Core Servisi  Geliştirmek ve Kullanmak                                           | _(=======>%70)_    |     M    |      Windows 10     | Pasif |
| 06 | DT-Training _(.Net Core Eğitimi için Hazırlık)_                                                              | _(==>%20)_         |    XL    | Heimdall Windows 10 | Aktif |
| 07 | Sıkça Duymaya Başladığım Deno'ya  Minik Bir Merhaba                                                          | _(========>%80)_   |     M    |       Heimdall      | Pasif |
| 08 | Yine Yeni Yeniden ELK _(Bu sefer E ve K için docker-compose işin içinde)_                                    | Tamamlandı         |     M    |       Heimdall      | Pasif |
| 09 | Distributed Cache Gerekiyorsa Elbette Redis                                                                  | Tamamlandı         |     S    |       Heimdall      | Pasif |
| 10 | Python Tarafında ProtoBuf Serileştirme                                                                       | Tamamlandı         |     S    |       Heimdall      | Pasif |
| 11 | Basit Bir .Net Core Worker Servisi  _(Linux Daemon Olarak)_                                                  | Tamamlandı         |     S    |       Heimdall      | Pasif |
| 12 | Sequelize Kullanılan Bir NodeJs  Rest Servisi Geliştirmek                                                    | _(========>%80)_   |     S    |       Heimdall      | Pasif |
| 13 | Bir .Net Core Web Api Bir Nodejs Bir Python  Servisi Bir Araya Gelip docker-compose'a  Misafirliğe Gitmişler | Tamamlandı         |    XL    |       Heimdall      | Pasif |
| 14 | Bir Asp.Net Core Web Api Projesinde Generic Repository Deseninin Kullanımı                                   | _(=======->%75)_   |     L    |       Heimdall      | Pasif |
| 15 |                                                                                                              |                    |          |                     |       |
| 16 |                                                                                                              |                    |          |                     |       |
| 17 |                                                                                                              |                    |          |                     |       |

## Hashtag

Örnekleri yaparken kendimce bir hashtag bilgilendirmesi de yapmaya çalışıyorum. Nelerden bashedilmiş sonradan hatırlayayım diye.

```text
#cSharp #dotNetCore #docker #dockerCompose #nodejs #deno #gRPC #workerService #dameon #rest 
#redis #distributedCache #razor #ruby #go #express #python #flask #postrgres #sqlite #migration 
#entityFramework #elasticSearch #kibana #logstash #sequilize #protobuf #unitTest #xUnit #mongoDb 
#repositoryPattern #genericRepository #dependencyInjection #async #await #curl
```
## Kolay Klasör Açma

Gerek saturday-night-works olsun gerek skynet, çok sık tekrar ettiğim işlerden birisi de örneğe ait klasörler _(genelde No ile başlayan ana klasör ile src ve assets ile ilgili alt dizinler)_ ile Readme dosyasını açmak. İşi biraz kolaylaştırmak adına easy_setup.sh isimli bash script dosyasını kullanmaya karar verdim. Şimdilik iş görüyor.

![bash_screen.png](./bash_screen.png)

## Çalışma Tekniğim

~~Ahch-To üzerindeki Skynet çalışmalarımda pomodoro tekniğini kullanıyorum. Buna göre genelde 22:00 sularında masa başına geçiyorum ve 4x25 dakikalık çalışma periyotları uyguluyorum. Her pomodoro arasında standart olarak 5 dakikalık dinlenme molası veriyorum. Zamanlayıcı için [tomato-timer](https://tomato-timer.com/) sitesinden yararlanıyorum.~~
