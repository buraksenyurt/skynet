/*
    Bu script dosyasında Neftonsoft Nuget paketini kullanacağız.
    Uygulama basitçe klasördeki dosyaların adlarını ve boyutlarını bir nesne listesinde toplayıp,
    JSON formatında ekrana yazdırmakta.
*/

//r ile dış assembly veya nuget paketini referans edebiliriz
#r "nuget: Newtonsoft.Json, 12.0.3"

using System;
using System.Linq;
using System.IO;
using Newtonsoft.Json;

// Dosya bilgilerini tutacağımız sınıf deseni
class FInfo
{
    public string Name { get; set; }
    public Int64 Length { get; set; }
}

// Script dosyasının olduğu klasörü alıyoruz. Dolayısıyla hangi klasörde çalıştırırsak orayı kullanacak.
var currentDir = new DirectoryInfo(Environment.CurrentDirectory);
// Dosya bilgilerinden bir kısmını kullanarak FInfo listesini üretiyoruz
var fileList = currentDir.EnumerateFiles().Select(f => new FInfo { Name = f.Name, Length = f.Length });
// Json formatında serileştiriyoruz
var jsonFileList = JsonConvert.SerializeObject(fileList);
// Sembolik olarak ekrana basıyoruz. Dosyaya yazdırabilir veya bir servise gönderebilirsiniz de.
Console.WriteLine(jsonFileList);