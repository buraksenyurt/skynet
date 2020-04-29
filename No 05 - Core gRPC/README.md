# gRPC Destekli Basit Bir .Net Core Servisinin Geliştirilmesi

gRPC üzerinden yayın yapan basit bir servis ve bunu yardımcı NuGet paketleri sayesinde tüketebilen çeşitli tipte istemcilerin geliştirilmesi amaçlanmıştır. Örnek Visual Studio 2019 üzerinde yazılmaktadır.

## Ön Hazırlıklar

```
docker
```

## Client App için gerekli NuGet Paketleri

```
Grpc.Net.Client
Google.Protobuf
Grpc.Tools
```

## Çalışma Zamanı

Önce CookingAPI servisi çalıştırılır. Ayarlamalar gereği http://localhost:5555 üzerinden hizmet verecektir. Ardından Apprentice istemcileri yürütülebilir.

![screenshot_01.png](./assets/screenshot_01.png)
