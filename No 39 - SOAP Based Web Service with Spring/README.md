# Spring Tarafında Eski Usül Soap Based Bir XML Web Service Yazmak Ne Kadar Zor Olabilir?

Kısa bir süre önce değerli bir çalışma arkadaşım kullanmaya çalıştığı Java tabanlı XML Web servis ile epeyce sorun yaşadı. Söz konusu servisi .Net tarafından tüketmeye çalışıyordu ancak XML şeması da epeyce karmaşık olan servis iletişim noktasında şema adlarına kızıyor, header içeriğini beğenmiyor sürekli naz yapıyordu. Arkadaşım allem etti kallem etti sorunun altından girip üstünden çıkıp çözdü. Bu olaylara kısmen tanıklık ettikten sonra yahu Java tarafında XML Web Service geliştirmek şimdilerde daha bir kolay değil midir diye düşünmeye başladım. Yol doğal olarak beni Spring'e ve resmi dokümantasyonuna götürdü. Amacım yönergeleri takip edip Spring ile basit bir XML Web servisinin nasıl yazıldığını deneyimlemek. Günümüzde neredeyse tüm servisler REST, gRPC, OData ve benzeri kavramlar üzerinde konuşuyor olsalar da özellikle kurumsal çapta ve uzun yıllardır hayatta olan pek çok uygulama halen daha SOAP _(Simple Object Access Protocol)_ temelli XML Web servislerini kullanıyor.

## Ön Hazırlıklar

Diğer spring çalışmalarında da olduğu gibi işe [https://start.spring.io/](https://start.spring.io/) adresine gidip projeyi üreterek başlamak gerekiyor. Daha önceki örneklerde olduğu gibi Maven kullanacağım. Bu sefer Spring Web, Spring Web Services paketlerine ve wsdl4j _(Java için WSDL kullanıcığımız bildireceğimiz dependency)_ tanımlamasına gereksinimimiz var. Detaylar için adres pom.xml...

![Screenshot_01.png](./assets/Screenshot_01.png)

## Çalışma Zamanı

## Bomba Sorular

## Ödevler