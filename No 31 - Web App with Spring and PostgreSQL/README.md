# Spring Boot ile PostgreSQL Kullanan Basit Bir Web Uygulaması Geliştirmek

Spring Boot maceralarım devam ediyor. Bu sefer PostgreSQL veritabanını kullanan bir Web uygulaması geliştirmek istiyorum. Nitekim Spring Boot kullanarak bir web uygulaması hiç geliştirmedim. PostgreSQL tarafında Heimdall sistemini kirletmemek adına docker container kullanacağım.

## PostgreSQL Hazırlıkları

Docker imajını ve örnek veritabanını hazırlamak için aşağıdaki terminal komutlarını kullandım.

```bash
# Container'ı Tokyo ismiyle ayağa kaldıralım
sudo docker run --name Tokyo -e POSTGRES_PASSWORD=P@ssw0rd -p 5432:5432 -d postgres
# Üzerinde bash açıp
sudo docker exec -it Tokyo bash
# PostgreSQL veritabanımızı oluşturalım
psql -U postgres
Create Database qworld;
```

## Uygulamanın İnşaası

İlk iş olarak [şu](https://start.spring.io/) adrese gidip POM içeriğini ve uygulamayı hazırlamak lazım. PostgreSQL tarafı için _PostgreSQL Driver_ , temel web uygulaması kabiliyetleri için _Spring Web_ , temel MVC şablonlarını kullanabilmek için _Thymeleaf_ ki bunu bir türlü telaffuz edemiyorum, Object Relational Map aracı Hibernate içinse _Spring Data JPA_ isimli bağımlılıkları yüklüyoruz. 

![Screenshot_01.png](./assets/Screenshot_01.png)

Sonrasında indirilen uygulama içeriği üzerinde gerekli geliştirmeleri yapıyoruz.

```bash

```

## Çalışma Zamanı

## Bomba Sorular

## Ödevler