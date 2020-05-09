# .Net Core 3 Tabanlı Basit Bir Razor Uygulaması

Şu Razor'u gözü kapalı kavramam lazım. Başlangıç noktası olarak Microsoft'un resmi dokümanı pekala yeterli.

## To Do

- Albüm silme de popup deneyelim
- Albümlere standart boyutta kapak fotoğrafı ekleyelim
- Ana sayfada günün albümü olsun. Rastgele gelsin.
- Albümlerde veya artistlerde arama seçeneği koyalım

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

![screenshot_1.png](./assets/screenshot_1.png)

**_Artistlerin görüntülendiği sayfa_**

![screenshot_2.png](./assets/screenshot_2.png)

**_Yeni bir artist eklenirkenki kullanılan sayfa_**

![screenshot_5.png](./assets/screenshot_5.png)

**_Albüm ekleme sayfasının ilk hallerinden_**

![screenshot_6.png](./assets/screenshot_6.png)

**_Albüm eklendikten sonra gelinen albümler sayfasının ilk hallerinden_**

![screenshot_3.png](./assets/screenshot_3.png)

**_Artistler sayfasında albümlerine götüren link eklendiğinde_**

![screenshot_4.png](./assets/screenshot_4.png)

**_Artist sayfasından bir artist seçildiğinde gidilen albüm sayfası_**
