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