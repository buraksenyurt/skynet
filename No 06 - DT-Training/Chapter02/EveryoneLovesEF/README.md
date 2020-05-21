# Hepimiz Databaseciyiz! :)

Kim ne derse desin günümüz yazılımcılarının özellikle kurumsal tarafta çalışanlarının çoğu Line of Business adı verilen uygulamalar üzerinde geliştirme yapıyor. Doğal olarak çoğumuz veritabanı ile ilgili işlere dahil oluyoruz. İster ilişkisel veritabanları ister NoSQL sistemleri olsun bir şekilde veriyle buluşuyor. Entity Framework yıllardır hayatımızda olan .Net'in bel bağladığı en önemli ORM _(Object Relaitonal Mapping)_ araçlarından birisi. En en en basit haliyle nasıl kullanıldığına bakalım.

>Bu örnekte dahili SQLite veritabanını model-based bir model için kullanabiliriz.  

## Başlangıç Adımları

Microsoft.EntityFramework.SQLite paketi gerekecektir.

```bash
dotnet new console -o Intro
cd Intro
dotnet add package Microsoft.EntityFrameworkCore.Sqlite
dotnet add package Microsoft.EntityFrameworkCore.Tools
```

## Migration İşlemleri

```bash
dotnet ef migrations add initial
dotnet ef database update
```

## Bölüm Soruları

_Hadi yine iyisiniz?_

## Mini Lab Çalışması _(Süre: Bir sonraki güne ödev)_

Çalışma odamdaki kütüphanede yer alan kitapları çok basit bir kurgu ile dijital ortamda takip etmek istiyorum. Kütüphanedeki bir kitabın adını, sayfa sayısını, yazarlarını ve okuyup okumadığımı hatırlamak benim için önemli. Ayrıca onların hangi rafta oldukları bilgisi ile hangi kategoride yer aldıklarını da kayıt altına almak istiyorum. Kütüphane tasarımını aşağıda bulabilirsin.
6 katlı ve her katında 4er bölmeli bir kitaplık. Koordinat noktalarını istediğin şekilde tasarlayabilirsin.

```text
[-----][-----][-----][-----]
[-----][-----][-----][-----]
[-----][-----][-----][-----]
[-----][-----][-----][-----]
[-----][-----][-----][-----]
[-----][-----][-----][-----]
```

Bunun için bana SQLite tabanlı Entity Framework kullanan bir Console uygulaması yazar mısın? Terminalden beklediğim işler şöyle.

```text
dotnet run

Hoşgeldin!
Ne yapmak istersin?
1 - Kütüphaneme yeni bir kitap ekleyeceğim?
2 - Şu kitabın hangi rafta olduğunu söyler misin?
3 - Kütüphanemden rastgele 10 kitap listeler misin?
4 - Arkadaşıma hediye ettiğim bir kitap var. Bunu kütükten çıkartır mısın?
```

başarılar :)
