# Rust Dilinde Warp, Tokio Küfelerini Kullanarak Asenkron Web Api Geliştirmek

Rust dilinin Message Passing ve Mutex<T> konularını öğrenmeye çalışırken karşıma Warp ve Tokio küfeleri _(Crates)_ çıktı. Derken olay asenkron çalışan bir Web API geliştirme olayına döndü. Warp, Rust için geliştirilmiş bir Web Server Framework. Sonuçta bir Web API söz konusu olduğundan bu tip bir kütüphane gerekiyor. Bakalım küfeden neler çıkacak? Tokio ise Rust dilinde asenkron çalışmayı kolaylaştırmakta. Amacım bu paketlerden yararlanarak asenkron çalışan bir Web API'nin Rust dilinde geliştirme temellerini anlamak.

## Ön Hazırlıklar

Ben örneği daha önceden de olduğu gibi Heimdall _(Ubuntu 20.04)_ üstünde geliştiriyorum. Sistemde Rust yüklü.

```bash
# İlk önce web api projesini oluşturalım
cargo new musician-api

# Gerekli Paketlerin Yüklenmesi
# Tokio, Warp ve JSON serileştirme için gerekli Serde paketleri 
# Cargo.toml içerisindeki Dependencies sekmesinde yer alıyorlar
# Dolayısıyla sonrasında build işlemi yapmak lazım
cd musician-api
cargo build

# Entity olarak bir struct kullanacağız
# Models isimli küfede Product ve 
# başkalarını konuşlandırabiliriz
touch models.rs

# Veritabanı tarafı
# Aslında in-memory çalışacan bir veri modelimiz var
# Bir json kaynağındaki veriyi okuyor
touch rust_lite.rs

# Product tipi ile ilgili CRUD operasyonlarını
# product-repository isimli dosyada toplayabiliriz
touch product_repository.rs

# Web API taleplerini yöneteceğimiz bir mekanizma da gerekiyor
# Bunu router.rs içinde toplayabiliriz
touch router.rs
```

## Çalışma Zamanı

Örnek kodu çalıştırdıktan sonra Postman, curl gibi araçları kullanarak çeşitli talepler gönderebiliriz.

```bash
cargo run
```

İlk olarak tüm ürünlerin listesini çekmeyi deneyelim.

```text
Adres : http://localhost:5555/products
Metot : HTTP Get
```

![Screenshot_01.png](./assets/Screenshot_01.png)

## Bomba Sorular

- Henüz yok

## Ödevler

- Henüz yok