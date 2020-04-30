# gRPC Destekli Basit Bir .Net Core Servisinin Geliştirilmesi

gRPC üzerinden yayın yapan basit bir servis ve bunu yardımcı NuGet paketleri sayesinde tüketebilen çeşitli tipte istemcilerin geliştirilmesi amaçlanmıştır. Örnek Visual Studio 2019 üzerinde yazılmaktadır. gRPC servisi PostgreSQL ile çalışan bir uygulamadır ve CRUD operasyonları için Entity Framework ile entegre edilmiştir.

## Ön Hazırlıklar

Verileri saklamak için Postgresql tercih ettim. Sisteme yüklemek yerine docker imajını kullanmak işime geldi.

```
docker run --name London -e POSTGRES_PASSWORD=P@ssw0rd -p 5433:5432 -d postgres
docker exce -it London bash
psql -U postgres

Create Database GeniusRecipes;
Select datname from pg_Database;
```

## Servis Uygulaması için Gerekli NuGet Paketleri

gRPC servisi PostgreSQL tabloları için Entity Framework kullanabilir.

```
Npgsql.EntityFrameworkCore.PostgreSQL
Microsoft.EntityFrameworkCore.Design
```

## EF Migration Komutları

Visual Studio 2019 üzerinde CookingAPI projesi seçiliyken Package Manager Console üzerinden çalıştırılan komutlardır. Böylece GeniusRecipesContext sınıfı baz alınarak Postgresql tarafındaki db nesneleri oluşturulur. Komutların çalışabilmesi için ilgili projede Microsoft.EntityFrameworkCore.Design paketinin yüklü olması gerekmektedir.

```
Add-Migration initial
Update-Database
```

## Client App için gerekli NuGet Paketleri

```
Grpc.Net.Client
Google.Protobuf
Grpc.Tools
```

## Çalışma Zamanı

Önce CookingAPI servisi çalıştırılır. Ayarlamalar gereği http://localhost:5555 üzerinden hizmet verecektir. Ardından Apprentice istemcileri yürütülebilir.

![screenshot_1.png](./assets/screenshot_1.png)

![screenshot_2.png](./assets/screenshot_2.png) 

```
docker exec -it London bash
\connect geniusrecipes
Select "Name","Calories" from "Recipes";
```

Verileri Postgresql tarafında analiz etmek için docker üzerinden bash'e geçip psql CIL aracı kullanılabilir ya da Azure Data Studio gibi çok şükela bir ürün de düşünülebilir.
