# .Net Core Tarafından RabbitMQ'ya Mesaj Göndermek Java Tarafından Dinlemek

Çok sık karşılaştığımız senaryolardan birisidir. Bir uygulama kendi bünyesinde gerçekleşen bir olay sonrası başka bir uygulamayı haberdar etmek ister ya da başka bir uygulamanın yaptıklarından haberdar olmak isteyen bir uygulama vardır. Bunun bir çok sebebi olabilir. Örneğin uygulamalar farklı teknolojilerde yazılmıştır ancak ortak iş süreçleri üzerinde koşmaktadır. Gerçek bir senaryo koyalım ortaya. Kargo çıkışı gerçekleştiren yeni nesil bir uygulama bu çıkışlar için düzenlediği irsaliyelerin bir devlet kurumuna gönderilmesi sırasında yine aynı kurumun legacy başka bir sisteminin süreçlerine dahil olmak durumundadır. Karşılıklı olarak asenkron bir iletişim gereklidir. Burada asenkron bir haberleşme olması kaçınılmazdır ve söz konusu olaylardan çok çok fazla sayıda işlem olabilir. Yani asenkron bir kuyruk sistemi epeyce iş görecektir.

Örnek çalışmadaki amacım RabbitMQ'yu Heimdall üstünde olabilecek en basit haliyle çalıştırmak, bir .Net Core Console uygulamasından belli bir topik için bu kuyruğa mesaj yollamak ve oldukça yabancısı olduğum Spring tarafındaki bir Java uygulamasından da gönderilen mesajları almak.

## Ön Hazırlıklar

Heimdall _(Ubuntu 20.04)_ yüklü bir RabbitMQ hizmeti yok. Aslında kurmayı da istemiyorum. Docker veya Docker-Compose kullanmak işime gelir.

```bash
# Bu nedenle bir docker-compose.yml oluşturup çalıştırsam yeterli olur
# İçine bakın tabii :)
touch docker-compose.yml
# Çalıştırmak için
docker-compose up
```

## Çalışma Zamanı

## Bomba Sorular

## Ödevler