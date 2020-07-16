# Şu Web Dedikleri Şey de Neymiş

.Net Core ile Web uygulamalarını birkaç şekilde geliştirebiliriz. Asp.Net Core Razor Pages ve MVC _(Model View Controller)_ temelli web siteleri en çok kullanılanlar arasındadır. İzleyen örneklerde çok temel seviyede Razor ve MVC uygulamaları oluşturulmaktadır. İlaveten konunun bütünlüğünü sağlamak amacıyla bir Web API projesi de eklenmiştir.

## Gerekli Northwind EF Uygulamasının İnşası

Her iki Web uygulaması da örnek bir EF _(Entity Framework)_ projesini referans ederek kullanmaktadır. Bu projeyi aşağıdaki şekilde oluşturup gerekli paket ve dosyaları ekleyebiliriz.

```bash
dotnet new classlib -o NorthwindLib
cd NorthwindLib
dotnet add package Microsoft.EntityFrameworkCore.SQLite
rm Class1.cs
touch Category.cs Product.cs Northwind.cs
```

Bu kütüphane Northwind için Product ve Category veri setlerini işaret edecek şekilde tasarlanmıştır. Sonraki Web projelerinde EF destekli istenen veritabanları ile kullanılabilir.

## Razor Pages Uygulamasının İnşası

Uygulamayı başlangıçta aşağıdaki gibi oluşturabiliriz.

```bash
# Razor projesini boş bir Web sitesi şeklinde oluşturuyoruz
dotnet new web -o GameWorldWeb

# Projeye HTTPS desteği eklendikten sonra gerekli sertifika tanımı
dotnet dev-certs https --trust

# Statik sayfa kullanımı örneği için eklendiler
mkdir wwwroot
touch wwwroot/index.html

# Razor sayfa örnekleri için eklendi
mkdir Pages
mv wwwroot/index.html Pages/index.cshtml

# Ortak Şablon için eklenir
touch Pages/_ViewStart.cshtml
mkdir Pages/Shared
touch Pages/Shared/_Layout.cshtml

# _Layout.cshtml yazıldıktan sonra tasarlanan sayfalar
touch Pages/companies.cshtml Pages/companies.cshtml.cs

# Entity Framework projesini kullanabilmek için diğer projeyi referans ediyoruz.
dotnet add reference ../NorthwindLib/NorthwindLib.csproj

# Migration işlemleri
# startup.cs içerisinde migration assembly seçimi yaptığımıza dikkat edin
dotnet add package Microsoft.EntityFrameworkCore.Design
dotnet ef migrations add initial
dotnet ef database update
```

## MVC Uygulamasının İnşası

Uygulamayı oluşturmak için aşağıdaki terminal komutu ile işe başlayailiriz.

```bash
dotnet new mvc --auth Individual -o GamerMVC

# NorthwindLib isimli Entity Projesini Kullanacağımız için Referans Ekleme
cd GamerMVC
dotnet add reference ../NorthwindLib/NorthwindLib.csproj

# {Controller}{Action}{ViewModel} şeklinde bir isimlendirme standardına göre model sınıfı oluşturulur
touch Models/HomeIndexViewModel.cs

# Firma adına basılınca açılacak olan View sayfası
touch Views/Home/CompanyGamesDetail.cshtml

# Yeni oyun firması eklemek için kullanılacak model tipi ve view
# CompanyGameModel'de amaç bir firma eklerken ilk ürettikleri oyunu da modelden almak
touch Models/CompanyGameModel.cs
touch Views/Home/CreateCompany.cshtml

# Yeni bir View ve Controller ikilisi ekliyoruz. Oyunları göstereceğiz. Hatta birde model koyalım.
mkdir Views/Game
touch Views/Game/Index.cshtml
touch Controllers/GameController.cs
touch Models/GameIndexViewModel.cs
```

Yukarıdaki gibi uygulamanın oluşturulması tamamlandıktan sonra _dotnet run_ komutu ile çalıştırıp kişisel e-posta adresimiz ile sisteme kayıt olabiliriz. Bu Membership tipi --auth Individual parametresi ile eklenmiştir. Porjenin ilk çalıştırılmasından sonra bir güzel sağı solu detaylıca anlatılır. Örneğin temel klasör yapısı ile başlanıp Dependency Injection'dan çıkılabilir.

- Areas klasöründe AspNet Core Identity için gerekli dosyalar durur.
- Bin klasöründe uygulamanın derlenmiş kütüphaneleri yer alır.
- Controllers klasöründeki sınıflar View içerisindeki sayfalar tarafından kullanılırlar ve genel olarak Model ile View arasındaki ilişkili ele alırlar.
- Data klasöründe Entity Framework Context sınıfı ve Migration planları yer alır. Varsayılan olarak proje ilk açıldığında SQLite kullanacak şekilde ayağa kalkar.
- Uygulamanın Kestrel, IIS gibi çalışma zamanı web ortamı yayıncıları ile ilgili bilgileri Properties klasöründeki launchSettings.json dosyasında tutulur.
- Views klasöründe C# ve HTML'in birlikte harmanlandığı Razor Pages dosyaları yer alır. Önyüzün inşaası burada gerçekleşir. Sayfaların çalışacağı modeller, Models dizininde yer alır.
- Bir web uygulamasının olmassa olmaz parçalarından birisi css, javascript gibi unsurlardır. Bunlar wwwroot altındaki ilgili klasörlerde yer alır.
- appsettings içerisinde SQLite veritabanının adı ve adresi yer alır. Bunun dışında Loglama ile ilgili ayarlar da bulunur.
- Uygulama varsayılan haliyle Microsoft Identity Server üzerinden Membership _(Üyelik Sistemi)_ yönetimini destekler. Register adımlarından sonra app.db içerisindeki AspNetUsers tablosuna kullanıcı atıldığı görülür.
- Startup.cs dosyasındaki ConfigureServices ve Configure metotları incelemeye değerdir. _(Eğitime katılanlarla buradaki kodlar üzerinde durulması gerekir)_
- Controller sınıflarının ihtiyacı olan servisler Constructor üzerinden enjekte edilirler. DotNetCore dahili Dependency Injection mekanizmasına sahiptir. Bu servislerin register edilmesi için ConfigureServices metodu kullanılır.
- URL yönlendirme mekanizması MapControllerRoute üzerinden sağlanır ve neredeyse her request ele alınabilir. Aşağıdaki tablo işi kavramak için yeterlidir.

| URL                         | Controller |  Action  |      ID     |
|-----------------------------|------------|:--------:|:-----------:|
| /                           | Home       |   Index  |             |
| /Player                     | Player     |   Index  |             |
| /Player/Zero                | Player     |   Zero   |             |
| /Games                      | Games      |   Index  |             |
| /Games/Detail               | Games      |  Detail  |             |
| /Games/Detail/791           | Games      |  Detail  |     791     |
| /Games/Blizzard/Hearthstone | Games      | Blizzard | Hearthstone |

>NotCompletedException();

## Web API Uygulamasının Geliştirilmesi

Konu bütünlüğü için eklediğimiz Web API projesini oluşturmak için aşağıdaki adımları takip edebiliriz. Bu örnekte de NorthwindGameCatalog isimli SQLite veritabanı kullanılmaktadır.

```bash
dotnet new webapi -o GameWorldApi
cd GameWorldApi

#Kullanacağımız Entity kütüphanesini ekledik
dotnet add reference ../NorthwindLib/NorthwindLib.csproj

#Şablonda varsayılan olarak gelen Controller sınıfını ve tipi çıkartıp kendi Controller sınıfımızı ekliyoruz
rm WeatherForecast.cs
rm Controllers/WeatherForecastController.cs
touch Controllers/CompanyController.cs

#Repository desenini tercih ettik. Bu nedenle bir klasör oluşturuyoruz ve içerisine dosya açıyoruz
mkdir Repository
touch Repository/ICompanyRepository.cs Repository/CompanyRepository.cs

#REST Client testleri için klasör oluşturup içerisine test dosyaları atıyoruz
mkdir RestTests
touch RestTests/get-companies.http RestTests/create-company.http RestTests/delete-company.http
```

>WebAPI Uygulamasında testler için humao.rest-client isimli bir Extension kullanılmıştır. Test klasöründeki http uzantılı dosyalara dikkat edin. WebAPI projesi çalışır haldeyken bu dosyalar içerisindeki Send Request linklerine basıp çağrı sonuçlarını VS Code üzerinden de görebiliriz.

![Screenshot_1.png](./assets/Screenshot_1.png)

### Çalışma Zamanı _(Tüm Örnekler İçin Kendi Klasörlerinde)_

```bash
dotnet run
```

yazmak yeterlidir. Ardından genellikle <https://localhost:5001> adresine gidilir. Web Api uygulaması için <https://localhost:5551/api/> üstünden denemeler yapılabilir.

## Bölüm Soruları

- Razor ve MVC projelerindeki Startup sınıfının görevi nedir?
- Projeleri SQLite yerine SQLServer ile çalışacak şekilde ayarlamak için nerelerde nasıl ayarlamalar yaparsınız?
- Companies Razor sayfası backend tarafına nasıl bağlanabildi?
- HSTS'in kullanım amacı nedir?
- GamerMVC örneğinde, View üstünden gelen bir action talebi kod tarafında nasıl karşılanır? Bir arkadaşınıza anlatın.
- GamerMVC örneğinde, Yeni Firma eklerken doğrulama kontrollerine takıldığımızda hata mesajlar ekran görünmesine rağmen son girilen veriler kontrollerden kayboluyorlar. Bunun sebebi nedir, nasıl çözümlenir?
- GamerMVC örneğinde Game View'undaki ana sayfada oyun şirketinin adını görünmüyor. Bunu göstermek için ne yapmak gerekiyor? Bulup uygulayınız. Sonrasında sonuçları arkadaşlarınıza gösterip hava atınız. 

>NotCompletedException();

## MiniLab Çalışması

- Razor projesinde yeni oyun ekleyebileceğimiz _(eklerken yapımcıyı da seçebileceğimiz)_ bir cshtml sayfası geliştiriniz.
- Razor projesinin anasayfasında rastgele günün oyun firması ve oyunlarını gösterecek geliştirmeleri yapınız.
- Game tablosundaki Popuplarity alan adını Popularity olarak düzenleyin.
- CompanyGamesDetail.cshtml sayfasında, oyunların sahibi olan firma adının da çıkmasını sağlayın.
- Game tablosunda oyuna ait özet bilgiyi tutabileceğimiz bir Description alanı ekleyip CompanyGamesDetail.cshtml sayfasında görünmesini sağlayın.
- GamerMVC örneğindeki Index sayfasında yılların gösterildiğini görüyorsunuz. Bir yıla bastığımda o yıla ait oyunları gösterecek geliştirmeyi yapınız.
- WebApi projesinde CompanyController'da eskik kalan Update operasyonunu tamamlayınız.
- Yine WebApi projesinde oyun kataloğundaki Game nesnelerinin yönetimi için Repository ve Controller sınıflarını siz yazıp geliştirmeye çalışınız. _(Yazarken Company nesnesi için yapılanlardan yardım alabilirsiniz)_

>NotCompletedException();
