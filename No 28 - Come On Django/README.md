# Django ile Bir Web Uygulaması Geliştirmek

Bir zamanlar IoT cihazlarda uygulama geliştirmek için Python programlama dilini öğrenmeye çalışmıştım. O vakitler Flask ile basit REST Servisleri deneyimleme fırsatım da olmuştu ama Django ile etraflıca uğraşmamış bir web sitesi/uygulaması geliştirmeyi denememiştim. İşte rövanş zamanı. Amacım Django'nun resmi dokümanlarını takip ederek basit bir web uygulaması geliştirme deneyimini yaşamak. Örneği favori dostum Heimdall _(Ubuntu 20.04)_ ile birlikte geliştireceğim.

## Yapılanlar

Elbette sistemde python ve pek tabii django çatısının yüklü olması gerekiyor.

```bash
# Sistemde python yüklü olduğundan aşağıdaki komut ile django framework'ü yükledim
python -m pip install Django

# Kurulumdan emin olmak için versiyon kontrolü yaptım
python -m django --version

# Örneğin src klasörüne geçip aşağıdaki komutu kullanarak web site'ı oluşturdum(Project olarak düşünelim)
# Standart olarak bir web site şablonu oluşur. Bu aslında proje klasörüdür. Web Site gibi düşünürsek içinde birden fazla web application içerebilir
django-admin startproject worldsite

# manage.py dosyası özellikle django işleri için kullanacağımız komut satırı programıdır
# ki örneği bu haliyle belli bir porttan çalıştırmak için onu aşağıdaki gibi kullanmak yeterlidir.
# Port bilgisi girilmezse 8000 portu kullanılır.
# Aşağıdaki işlem sonrası komut satırında migration tarafının tamamlanmadığı uyarısı ile birlikte http://localhost:65001 adresinde uzaya doğru hareket eden bir roket figürü görülmelidir ;)
cd worldsite
python manage.py runserver 65001

# Şimdi bu site altında bir web uygulaması oluşturalım. Yukarıda belirtmiştim. Site birden fazla web uygulaması içerebilir ve herbirinin konfigurasyon yönetimi buradan yapılabilir
python manage.py startapp quoteworld

# quoteworld içerisinde örnek bir view oluşturulur.
cd quoteworld
touch views.py

# Ayrıca url map tanımları için birde urls.py dosyası eklenir.
touch urls.py





# Gerekli düzenlemelerden sonra tekrar website sebiyesinde sunucuyu çalıştırıyoruz
cd ..
python manage.py runserver 65001
```

## Çalışma Zamanı

quoteworld web uygulaması sunucu çalıştırma şeklimiz gereği http://localhost:65001/quoteworld adresi üzerinden hizmet verecektir. 

![Screenshot_01.png](./assets/Screenshot_01.png)

## Bomba Sorular

## Ödevler