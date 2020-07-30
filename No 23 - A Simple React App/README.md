# Basit Bir React Uygulaması Geliştirmek _(Typescript, NodeJs, Express ve MongoDB ile)_

Başlıktan da anlaşılacağı üzere konumuz Typescript _(Hem sunucu hem istemci tarafında kullanacağız)_ , NodeJs ve MongoDb kullanan bir React uygulaması geliştirmek.

## Kurulum ve Hazırlıklar

### Sunucu(NodeJS) Tarafının İnşaası

Heimdall üzerinde bu kez npm yerine yarn'dan faydalandım.

```bash
mkdir gamer
cd gamer

yarn init

#Proje iskeletinin oluşturulması

#Dağıtım klasörü
mkdir dist

#Source Code klasörü
mkdir src

#Sunucu uygulamanın giriş noktası
touch src/app.ts

#Model bölümü
mkdir src/models
touch src/models/player.ts

#Controller bölümü
mkdir src/controllers
mkdir src/controllers/players
touch src/controllers/players/index.ts

#Router bölümü
mkdir src/routes
touch src/routes/index.ts

mkdir src/types
touch src/types/player.ts

#gerekli konfigurasyon dosyası (Derleyici için gerekli bilgileri dolduracağız)
touch tsconfig.json

#NodeJs(Sunucu) Tarafında Typescript kullanabilmek için gerekli yükleme
yarn add typescript

#NodeJs sunucu tarafındaki servis yetenekleri için express ve cors paketlerinin yüklenmesi
#Ek olarak Mongodb ile konuşabilmemizi sağlayacak mongoose paketinin yüklenmesi
yarn add express cors mongoose

#Typescript tarafının üstteki paketlerdeki tipleri anlayabilmesi için de aşağıdaki komut
yarn add -D @types/node @types/express @types/mongoose @types/cors

#tsc ile compile yapılacağının ve app.js'in eş zamanlı olarak nasıl başlatıldığının bilgisi package.json dosyasındaki scripts bölümünde gizli ;)
yarn add -D concurrently nodemon
```

### MongoDB için

MongoDB tarafı için elbette ki docker kullanacağım. Aşağıdaki gibi container'ı ayağa kaldırabiliriz.

```bash
docker run -d -p 27017-27019:27017-27019 --name gondor mongo
```

### İstemci Tarafı(React) Uygulamasının İnşaası

>throw NotStarted();

## Çalışma Zamanı

Önce sunufu tarafını çalıştırmamız gerekiyor. gamer klasöründeyken terminalden _yarn start_ dememiz yeterli. Bu aşamada eğer istemci uygulamayı henüz yazmadıysanız _curl_ veya _postman_ ile sunucuyu kullanarak get, add, delete testlerini yapabilirsiniz.

![Screenshot1.png](./assets/Screenshot1.png)

## Bölümün Bomba Sorusu

## Ödevler