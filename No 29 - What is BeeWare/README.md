# BeeWare ile Linux Platformunda Desktop Uygulaması Geliştirmek

Python ile ilgili bir şeyler ararken BeeWare isimli bir çalışmaya rastladım. Her yerde python ile native uygulama geliştirmek gibi bir felsefesi vardı. _(Eee zaten python her platformda yüklenip kullanılmıyor mu? Yok, öyle değil aslında)_ BeeWare ürünü macOS, Linux, Windows platformunda native uygulamalar geliştirmek haricinde iOS ve Android için de destek sunan bir araçlar ve kütüphaneler topluluğu esas itibariyle. Söz gelimi iOS ve macOS platformundaki Objective C kütüphaneleri ile Python arasında köprü görevi gören _Rubicon ObjC_ isimli bir araç sunuyor. Java kütüphaneleri ile bir iletişim mi söz konusu. O zaman _Rubicon Java_ var. Diğer yandan cross-platform için _Toga_ isimli bir widget kütüphanesi kullanıyor. Ayrıca python projelerini tek başına çalışabilir uygulamalar haline getirmek için _(standalone native application)_  _Briefcase_ isimli bir aracı var. Şöyle düşünebiliriz; Android için Gradle çıktısı, iOS için XCode proje çıktısı, Linux için AppImage, Windows için MSI Installer ve macOS İçin doğrudan çalışabilir uygulama çıktıları üretebiliyoruz. Bana bir Xamarin kokusu verdi gibi ama du bakalım. _(Bu arada BeeWare'in Logo'su acayip tatlı)_ İddialı bir platform. Benim amacım Heimdal _(Linux 20.04)_ üzerinde bir masaüstü uygulaması geliştirmek.

## Ön Hazırlıklar

```bash
# Sistemde Python yüklü olsa bile ekstra bazı kütüphaneler de gerekiyor
# Lakin bu paketleri hangi amaçla yüklüyoruz, araştırmam lazım. Mazallah güvenlik açığı filan da olabilir. Aman dikkat!
sudo apt-get install libgirepository1.0-dev libcairo2-dev libpango1.0-dev libwebkit2gtk-4.0.37 gir1.2-webkit2-4.0

# Şimdi Python paketinin dağıtımında devreye girecek Briefcase aracını yükleyelim
# Bu arada 20.04 üstünde cookiecutter versiyonunu beğenmedi Heimdall. O nedenle cookiecuttor'ı da pip üstünden install ettim
python3 -m pip install briefcase

# Adettendir kurulan versiyonu bir kontrol etmek iyi olabilir
briefcase --version

# Şimdi yeni projenin açılışını yapabiliriz
briefcase new

# Sorulan sorulara verdiğim cevaplar doğrultusunda cardgame isimli bir proje oluştu. 
# Carg Game isimli projenin GUI framework olarak Toga'yı seçtim. 
# Buna göre projenin Linux, macOS, Windows dağıtımlarındaki gereksinimleri ile birlikte
# diğer sorduğu sorulara() verdiğim cevaplar pyproject.toml içerisine yazıldı.
# Bu dosyayı incelemekte yarar var.
```

## Çalışma Zamanı

```bash
# linux, macOS, Windows... Hepsinde aşağıdaki şekilde.
cd cardgame
briefcase dev
```

## Bomba Soru

## Ödevler