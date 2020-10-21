# Docker Yerine Podman _(Pod Manager)_ Kullanmak

Bugün Heimdall üstünden birşeyler kurcalamak istediğimde ilk olarak docker imajı olup olmadığına bakıyor _(kuvvetle muhtemel buluyor)_ ve hemen container'ı ayağa kaldırıp denemelere başlıyorum. Ya da yeni bir ortam hazırlamak istediğimde Dockerfile'ı hazırlıyor ve imajını inşa ettiriyorum. Container teknolojileri denince çoğumuzun aklına Docker'dan başka bir şey gelmiyor. _"Gerçekten böyle mi?"_ diye düşünürken Podman isimli araçla karşılaştım ve Docker'ın güçlü bir alternatifi olarak nasıl kullanıldığını öğrenmem gerektiğine karar verdim.

Esasında Docker'ı tek bir container aracı olarak düşünmemek lazım. Sonuçta Open Container Initiative tarafından belirlenmiş standartlara uyan araçlar mevcut. Open Container Initiative üç temel özelliğin olmasını vurguluyor. Container çalışma zamanı _(runtime)_ , dağıtım stratejisi _(distribution)_ ve images _(images :D )_ Podman'de bu standartlara uyan bir araç. Yani Podman ile hazırlanan imajlar Docker ile de uyumlu diyebiliriz.

Red Hat tarafından açık kaynak geliştirilen Podman özünde Pod _(Hani Kubernetes'in en küçük işlem birimi)_ sistemine dayanıyor. Dolayısıyla Kubernetes'e göç etmek _(migration)_ kolay. Pod'lar içerisinde birden fazla Container söz konusu olabilir. Docker gibi deamon'a ihtiyaç duymuyor. Root kullanıcıyı da şart koşmuyor. Standart bir kullanıcı söz konusu ise onun için açtığı bir namespace'i kullanıyor. Diğer yandan öğrendiğim kadarıyla sadece Linux sistemlerde çalışıyor. Bununla birlikte Docker Compose'un karşılığı henüz yok _(Doğrulanmamış bilgi)_

## Kurulum

Tabii ilk olarak Heimdall _(Ubuntu 20.04)_ üstünde Podman'i kurmam lazım. 

>Güncel kurulum bilgilerine [https://podman.io/getting-started/installation](https://podman.io/getting-started/installation) adresinden bakılabilir.

```bash
# Podman resmi dokümantasyonundaki adımları takip ederek kurulumu yaptım
. /etc/os-release
echo "deb https://download.opensuse.org/repositories/devel:/kubic:/libcontainers:/stable/xUbuntu_${VERSION_ID}/ /" | sudo tee /etc/apt/sources.list.d/devel:kubic:libcontainers:stable.list
curl -L https://download.opensuse.org/repositories/devel:/kubic:/libcontainers:/stable/xUbuntu_${VERSION_ID}/Release.key | sudo apt-key add -
sudo apt-get update
sudo apt-get -y upgrade 
sudo apt-get -y install podman

# Sonrasında bir versiyon kontrolü de yaptım
podman -v
```

## Çalışma Zamanı

Birkaç komutla Podman'i incelemeye başlayalım.

```bash

```

## Bomba Sorular

## Ödevler