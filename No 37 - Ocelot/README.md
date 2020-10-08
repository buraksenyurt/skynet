# Ocelot Denemesi

Uzun süre önce bankada çalışırken nereye baksam servis görüyordum. Bir süre sonra ana bankacılık uygulaması dahil pek çok ürünün kullandığı sayısız servisin yönetimi zorlaşmaya başladı. Bir takım ortak işlerin daha kolay ve etkili yönetilmesi gerekiyordu. Müşterek bir kullanıcı doğrulama ve yetkilendirme kontrolü _(authentication & authorization)_, yük dengesi dağıtımı _(load balancing)_, birkaç servis talebinin birleştirilmesi ve hatta birkaç servis verisinin birleştirilerek döndürülmesi _(aggregation)_, servis verisinin örneğin XML'den JSON gibi farklı formata evrilmesi, servis geliş gidişlerinin loglanması, yönlendirmeler yapılması _(routing)_, performans için önbellekleme yapılması _(caching)_, servis hareketliliklerini izlenmesi _(tracing)_, servislerin kolayca keşfedilmesi _(discovery)_ , çağrı sayılarına sınırlandırma getirilmesi, bir takım güvenlik politikalarını entegre edilmesi, özelleştirilmiş delegeler yazılması _(custom handler/middleware)_ , tüm uygulamalar için ortak bir servis geçiş kanalının konuşlandırılması ve benzerleri. Sonunda Java tabanlı WSO2 isimli bir API Gateway kullanılmasına karar verildi. Geçtiğimiz günlerde de yine konuşma sırasında [Ocelot](https://github.com/ThreeMammals/Ocelot) isimli C# ile yazılmış açık kaynak bir ürünün adı geçti ve tabii ki bende bir merak uyandı. Kanımca hafif siklet mikroservis veya servis odaklı mimari çözümlerinde düşünülebilir. Ama önce denemem lazım.

## Senaryo

Örnekte şöyle bir senaryoyu icra etmeye çalışacağım. Bir oyuncunun farklı platformlardaki skorlarını ve kazandığı çeşitli promosyonları farklı servislerden çektiğini varsayıyorum. İstemci uygulama _(sembolik olarak bir Console uygulaması olabilir)_ bu birkaç servis çağrısı için API Gateway'e gelecek. Yani servislerle doğrudan değil API Gateway üzerinden konuşacak. Sonrasında API Gateway'in kontrol altında tuttuğu bu servisler için neler yapabileceğini keşfetmeye çalışacağım.

## Ön Hazırlıklar

Hayali olarak birkaç servise ihtiyacım var. Tamamını .net core web api olarak tasarlamak işime geliyor. Ancak gerçek hayat senaryosunda farklı programlama dilleri ve çatıları ile geliştirilmiş servisler kullanmak daha mantıklı olacaktır.

## Çalışma Zamanı

## Bomba Sorular

## Ödev