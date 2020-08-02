# GO'da Minik Bir CRUD Servisini Gin-Gonic ile Geliştirmek

Gin-Gonic hafif siklet sayılan ama yüksek performansı ile öne çıkan _(muadili sayılan martini'den çok daha hızlı olduğu ifade ediliyor)_ bir web framework. Elbette açık kaynak. Middleware tarafında _(Yani Request ve Response'lar arasında)_ Recover ve Log desteği sunuyor. Tabii kendi middleware bileşenimizi yazıp ekleyebiliriz de. Recovery modülü en başından beri ekli olduğundan paniklemeyen bir framework :) Yani Go çalışma zamanında HTTP request'leri ile ilgili olarak bir panic oluştuğunda uygun bir 500 cevabı verebiliyor. Söylentilere göre bu özelliği sayesinde söz konusu servis her an ayakta ve çalışır durumda kalıyor. Bunlara ilaveten yönlendirmeleri _(routes)_ gruplandırabiliyoruz ki bu da örneğin versiyonlamayı kolaylaştırıyor. Bu kısa notlar yeterli. Sahada deneyimlemek lazım. Amacım mongodb üzerinde basit crud işlemlerini yaparken gin-gonic üstüne kurulmuş golang tabanlı bir servis geliştirmek. _(MongoDB için docker imajı kullanacağım)_

## Kurulum

```bash
# Ana klasörümüz ve gerekli go dosyaları oluşturulur
mkdir book-worm-api
cd book-worm-api
touch main.go

# MongoDB tarafıyla eşlecek entity için
mkdir entity
touch entity/quotation.go

# CRUD Operasyonları için
mkdir data
touch data/quotation-repository.go

# gin-gonic ve diğer modüllerin yönetimi için
# mod uzantılı bir dosya oluşacaktır. Burada yüklediğimiz paket bilgilerini görebiliriz. 
go mod init book-worm-api

# gerekli paketlerin yüklenmesi gin-gonic ve mongodb için
go get -u github.com/gin-gonic/gin go.mongodb.org/mongo-driver

# mongodb docker container'ının çalıştırılması
# bookworms isimli bir veritabanını da oluşturtuyoruz (MONGO_INITDB_DATABASE)
# Ayrıca bu veritabanı için Root kullanıcı ve yetki de verdik
# Mongo'yu 27019 nolu porttan açtık 
docker run --name camus -e MONGO_INITDB_ROOT_USERNAME=admin -e MONGO_INITDB_ROOT_PASSWORD=P@ssw0rd -e MONGO_INITDB_DATABASE=bookworms -p 27017-27019:27017-27019 -d mongo:latest
```

## Çalışma Zamanı

![Screenshot_1.png](./assets/Screenshot_1.png)

## Bölümün Bomba Sorusu

## Ödevler