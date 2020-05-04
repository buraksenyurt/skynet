# .Net Core 3.0 Tabanlı Basit Bir Razor Uygulaması

Şu Razor'u gözü kapalı kavramam lazım. Başlangıç noktası olarak Microsoft'un resmi dokümanı pekala yeterli. 

## Platform

Örnek uygulama Windows 10 üzerinde Visul Studio Code kullanılarak geliştirilmiştir, veri tabanı için Docker imajından yararlanılmıştır.

## Uygulamanın İnşası

```
dotnet new webapp -o MusicShop
dotnet dev-certs https --trust
```
## Ortam İçin Gerekli Paket ve Araç Kurulumları

```
dotnet tool install --global dotnet-ef

dotnet add package Microsoft.EntityFrameworkCore.SQLite
dotnet add package Microsoft.EntityFrameworkCore.Design
```

## EF Migration Komutları

```
dotnet ef migrations add InitialCreate
dotnet ef database update
```

## Çalışma Zamanı