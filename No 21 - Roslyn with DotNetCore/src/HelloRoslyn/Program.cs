
using System;
using System.IO;
using System.Threading.Tasks;
// Roslyn için gerekli namespace bildirimleri
using Microsoft.CodeAnalysis.Scripting;
using Microsoft.CodeAnalysis.CSharp.Scripting;

namespace HelloRoslyn
{
    class Program
    {
        /*
            Roslyn operasyonları awaitable fonksiyonlardır.
            Bu nedenle main Main metodu async olarak imzalanamıştır.
        */
        static async Task Main(string[] args)
        {
            #region Çalışma zamanında kod yürütme

            // while (true) // sonsuz döngü
            // {
            //     try
            //     {
            //         Console.WriteLine("Çok basit bir kod parçası girip çalıştırmayı deneyin");

            //         // Komut satırından baya baya bildiğimiz C# kodu istiyoruz
            //         var yourCode = Console.ReadLine();

            //         /*
            //             Girilen kod içeriğini çalıştırıyoruz (Evaluate)
            //             Hatta yazılan kodlar için System isim alanını varsayılan olarak ekliyoruz. 
            //             Nitekim eklemediğimiz isim alanlarını kod satırında belirtmek durumundayız.
            //         */
            //         var executionResult1 = await CSharpScript.EvaluateAsync(yourCode, ScriptOptions.Default.WithImports("System"));

            //         Console.WriteLine($"{executionResult1}");
            //     }
            //     catch (CompilationErrorException e) // Komut satırından girilen kodlar için derleme hatası söz konusu olabilir. Bunu CompilationErrorException ile kontrol altına alabiliriz.
            //     {
            //         Console.WriteLine(e.Message);
            //     }
            // }

            #endregion Çalışma zamanında kod yürütme

            #region Birden fazla satırı alma

            /*
                Bu bölümdeki amaç çalışma zamanındaki kodları tek satıra yazmak yerine satır satır yazıp ilerletebilmek.
                Başlangıç RunAync çağrısı ile yapıldıktan sonra bir state ya da context oluşuyor.
                Döngüyle komut satırından alınan her kod satırı ContinueWithAsync ile bir önceki kod satırlarına ilaveten işletiliyor.
                quit yazıp döngüden çıktığımızda da, yazdığımız kodların çıktısını görüyoruz.
            */

            // Console.WriteLine("Sonlandırmak için quit yazın");
            // //Başlangıç durumunu hazırladık ve birkaç Namespace'i ilave ettik
            // var codeState = await CSharpScript.RunAsync("", ScriptOptions.Default.WithImports("System", "System.Collections.Generic"));
            // while (true)
            // {
            //     var command = Console.ReadLine();
            //     if (command == "quit")
            //         break;

            //     codeState = await codeState.ContinueWithAsync(command);

            // }
            // Console.WriteLine(codeState.ReturnValue);

            #endregion Birden fazla satırı alma

            #region Bir text dosyasındaki C# kodunun yüklenip çalıştırılması

            // utility code içeriğine dikkat edelim. Script dili gibi yazılmıştır.
            var codeContent = await File.ReadAllTextAsync("utility.code");

            /*
                Create, parametre olarak gelen kod parçasının derlenmesi içindir.
                Bu örnekte fark edilecek bir sonuç üretmez ama bir kod parçasının
                sıkça çalıştırılması gerektiği hallerde performans amacıyla kullanılır.
            */
            var codeScript = CSharpScript.Create(codeContent); // Compile
            var response = await codeScript.RunAsync(); //Execute

            // Aşağıdaki kullanım şekli bildiğim çalıştırma işlemidir. Evaluate edip sonucu yazdırırız.
            //var result = await CSharpScript.EvaluateAsync(codeContent);
            //Console.WriteLine(response);

            #endregion Bir text dosyasındaki C# kodunun yüklenip çalıştırılması
        }
    }
}
