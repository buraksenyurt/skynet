# .Net Core 3 Tabanlı Basit Bir Razor Uygulaması

Şu Razor'u gözü kapalı kavramam lazım. Başlangıç noktası olarak Microsoft'un resmi dokümanı pekala yeterli.

## To Do _(Yapılabilecekler)_

- Albüm silme de popup deneyebiliriz
- Albümlere kapak fotoğrafları için özel doğrulama(Custom Validation) koyabiliriz
- Albümlerde veya artistlerde arama seçeneği olabilir
- Albümlerdeki parçaları içeren yeni bir entity sisteme kazandırılıp ana sayfada günü şarkısı gösterilebilir

## Platform

Örnek uygulama Visul Studio Code kullanılarak geliştirilmiştir, veri tabanı olarak SQLite kullanılmıştır.
.Net Core 3.1 Runtime hedef alınmıştır.

## Uygulamanın İnşası

```bash
dotnet new webapp -o MusicShop
dotnet dev-certs https --trust
```

## Ortam İçin Gerekli Paket ve Araç Kurulumları

```bash
dotnet tool install --global dotnet-ef

dotnet add package Microsoft.EntityFrameworkCore.SQLite
dotnet add package Microsoft.EntityFrameworkCore.Design
```

## EF Migration Komutları

```bash
dotnet ef migrations add InitialCreate
dotnet ef database update
```

## Çalışma Zamanı

```bash
dotnet run
```

sonrasında localhost:5001 veya 5000 üstünden testlere başlanabilir.

![screenshot_10.png](./assets/screenshot_10.png)

**_Ana sayfada rastgelen albüm getirilmesi sonrası_**

![screenshot_3.png](./assets/screenshot_3.png)

**_Artistlerin görüntülendiği sayfa_**

![screenshot_2.png](./assets/screenshot_2.png)

**_Yeni bir artist eklenirkenki kullanılan sayfa_**

![screenshot_8.png](./assets/screenshot_8.png)

**_Albüm ekleme sayfası_**

![screenshot_7.png](./assets/screenshot_7.png)

**_Albüm eklendikten sonra gelinen albümler sayfası_**

![screenshot_9.png](./assets/screenshot_9.png)

**_Artist sayfasından bir artist seçildiğinde gidilen albüm sayfası_**
