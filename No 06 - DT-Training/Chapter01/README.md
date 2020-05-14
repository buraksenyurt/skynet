# I Hate 'Hello World' Uygulaması

## Komutlar

```bash
dotnet new help
dotnet new sln -o ElvisGoesOn
dotnet new xunit -o MusicBox.Test
dotnet new classlib -o MusicBox
dotnet sln add ./MusicBox
dotnet sln add ./MusicBox.Test/
cd MusicBox.Test
dotnet add reference ../MusicBox/MusicBox.csproj
dotnet build
cd ..
dotnet test
```

>Eğer Visual Studio Code tarafında Go To Definition gibi seçimler çalışmazsa View->Command Palette->OmniSharp: Select Project ile var olan projeyi seçmemiz sorunun çözümünde yardımcı olacaktır.

## Bölüm Soruları

- dotnet terminal komutu kullanımlarına 3 örnek yazınız. _(help, --version hariç )_
- dotnet Core ile hangi tipte projeler geliştirebiliriz?

## Mini Lab Çalışması

Aşağıdaki solution iskeletini oluşturup devam eden kod parçasını geliştiriniz. _(Süre 15 dakika)_

```text
Alfabe(Solution)
--->Alfabe.Entity (Class Library)
--->Alfabe.Service (Class Library)
--->Alfabe.ClientApp.Test (xUnit Test Project)
```

Test Senaryosu : Service sınıfından çağırdığımda bir metod ile aşağıdaki Kategori listesini çekebilmeliyim.
Kategori Listem : Kitap(10 adet), Film (20 Adet), Çay Bardağı (12 Adet)
