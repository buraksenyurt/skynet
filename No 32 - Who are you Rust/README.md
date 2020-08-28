# Stackoverflow Anketlerinin Yıllardır Bir Numara Çıkan Programlama Dili Rust Kimdir?

Stackoverflow'un [2020 yılı geliştirici anketine göre](https://insights.stackoverflow.com/survey/2020#technology-most-loved-dreaded-and-wanted-languages-loved) en sevilen programlama dilinin Rust olduğunu söyleyebiliriz. Hatta bu son yıllarda hep böyle. Nedir onu bu kadar özel yapan merak ediyorum. Bunu anlamanın tek yolu onunla bir şeyler karalamak. Resmi dokümantasyonuna göre ilk tespitlerim şunlar.

- İlk söylenmesi gereken şey Rust'ın amacının sistem seviyesinde programlama yapmak olduğu. C ve C++ gibi bir dil olduğunu düşünebiliriz.
- Şu sıralar çok popüler olmaya başlayan Deno'nun Rust ile yazıldığını söylesem...Ya da Microsoft Azure IoT Edge'in çok büyük bir kısmının onunla yazıldığını ifade etsem. İşletim sistemleri _(TockOS, Tifflin, RustOS, QuiltOS, Redox)_, oyun motorları, derleyiciler, container'lar, VM'ler, Linux dosya sistemleri vs, ls komutunun alternatifi olan exa vs... Yani Rust ile yazılım ve yazılım platformları geliştirildiğini ifade edebiliriz. Bu nedenle Rust donanım odaklı bir dil desek yeridir. Donanımı etkin kullanmaya çalışır.
- Rust ortamında Garbage Collector gibi bir mekanizma yoktur. Amaç çalışma zamanı performansının artırılmasıdır. Dilin hedeflerinden birisi de hızdır zaten.
- Diğer iki önemli hedefi de eş zamanlılık _(Concurrency)_ ve güvenli bellek kullanımıdır.
- Rust derlemeli bir dildir. Hatta derleme çıktısı WebAssembly'da olabilir.
- Pek çok diğer modern dilde olduğu gibi Rust'ın da etkili bir paket yönetim mekanizması vardır. İsmi de gayet makul ve mantıklı. _Cargo_
- Dilin arkasında Mozilla Labs'ın gücü var. Hatta [servo isimli yüksek performans vaat eden tarayıcı motoru](https://servo.org/) da Rust ile geliştirilmiş.
- Dilin diğer karekteristik özelliklerini elbette kod üstünde anlamaya çalışacağım.

## Kurulum

Önce Rust ortamını hazırlamak lazım. Ben Heimdall _(Ubuntu 20.04)_ üstünde ilerliyorum. Geliştirmeler için Visual Studio Code'dan yararlanacağım.

```bash
curl https://sh.rustup.rs -sSf | sh

# Dilin genel özelliklerini tanımak için bir dosya üstünde çalışalım
touch WhoAreYouRust.rs
```

## Örneklerden Anahtar Notlar

- factorial; 
    - mutable değişken tanımlama,
    - recursive metot parametresi için match kullanımı,
    - kütüphane bildiriminin nasıl yapıldığı
    - ekran girdisinin parse edilmesi
- lucky_number;
    - harici kütüphane nasıl bildirilir _(toml)_,
    - for döngüsünde aralık bildirimi,
    - parse sonucunun match ile ele alınması,
    - continue, break kullanımı,
    - compare işlem sonucunun match ile ele alınması,

## Çalışma Zamanı

```bash
# Rust kodlarını derlemek için 
rustc WhoAreYouRust.rs

# Çalıştırmak içinse
./WhoAreYouRust

# Cargo'dan bahsetmiştik (Kargo grubu geldi aklıma. Ne dinlerdim ama?)
# Cargo ile derleme, paket yönetimi ve daha bir çok işlem yapılabiliyor.
# Örnekleri Cargo ile geliştireceksek
# Klasör yapısını inceleyin ve toml dosyasına bakın. Projenin genel özellikleri ile bağımlı olduğu diğer paketler burada yer alacak.
# Kodlar src altındaki main.rs'tedir.
cargo new factorial
cd factorial

# Cargo üstünde build için
cargo build
# ve çalıştırmak için
cargo run

# Derleme yapmadan kodu kontrol etmek için
cargo check

# Release almak için
cargo build --release

# factorial örneğinde rand isimli rastgele sayı üretme kütüphanesinin kullanımı için toml dosyasında değişiklik yapıldı. (Bul bakalım)
# rand kütüphanesinin 0.5.3 sürümünü kullandık. Ek kütüphaneler cargo build komutu ile indirilir. Güncellenmeleri gerektiğinde cargo update komutu kullanılabilir.
```

> factorial sonrası geliştirilen diğer örneklerde cargo aracından yararlanılmıştır

_factorial programına ait örnek ekran çıktısı_

![Screenshot_01.png](./assets/Screenshot_01.png)

_lucky _ number isimli sayı tahmin oyunundan iki görüntü_

![Screenshot_02.png](./assets/Screenshot_02.png)

![Screenshot_03.png](./assets/Screenshot_03.png)

## Bomba Sorular

- Rust dilinde değişkenler neden varsayılan olarak immutable işaretlenir?
- factorial örneğindeki expect fonksiyonları hangi hallerde devreye girer? panic durumları bu kod parçasında nasıl ele alınır?
- lucky_number örneğindeki match kullanımlarının ne işe yaradığını bir arkadaşınıza anlatınız?

## Ödevler

- lucky_number örneğindeki cpm işlem sonucunu match yerine if blokları ile tesis ediniz.