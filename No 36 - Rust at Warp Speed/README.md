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

```

## Çalışma Zamanı

## Bomba Sorular

## Ödevler