
using System;
using System.Linq;
using System.IO;
using System.Threading.Tasks;
using System.Collections.Generic;
// Roslyn için gerekli namespace bildirimleri
using Microsoft.CodeAnalysis.Scripting;
using Microsoft.CodeAnalysis.Text;
using Microsoft.CodeAnalysis.CSharp.Scripting;
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.CSharp.Syntax;

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

            // // utility code içeriğine dikkat edelim. Script dili gibi yazılmıştır.
            // var codeContent = await File.ReadAllTextAsync("utility.code");

            // /*
            //     Create, parametre olarak gelen kod parçasının derlenmesi içindir.
            //     Bu örnekte fark edilecek bir sonuç üretmez ama bir kod parçasının
            //     sıkça çalıştırılması gerektiği hallerde performans amacıyla kullanılır.
            // */
            // var codeScript = CSharpScript.Create(codeContent); // Compile
            // var response = await codeScript.RunAsync(); //Execute

            // // Aşağıdaki kullanım şekli bildiğim çalıştırma işlemidir. Evaluate edip sonucu yazdırırız.
            // //var result = await CSharpScript.EvaluateAsync(codeContent);
            // //Console.WriteLine(response);

            #endregion Bir text dosyasındaki C# kodunun yüklenip çalıştırılması

            #region Syntax Tree Mevzusu

            /*
                Roslyn'in önemli avantajlarından birisi de, text tabanlı bir kod parçasının ağaç yapısını çıkartabilmesidir.
                Bu sayede statik kod analizi yapabiliriz. 
                Ayrıca LINQ yetenekleri sayesinde ağaç yapısında tarama işlemleri de kolaylaşır.

                Aşağıdaki senaryoda Utility dosyasının içeriğini analiz etmeye çalışıyoruz.
                Namespace ve içindeki sınıfların adları ile sınıfların kaç üyesi olduğunu,
                daha da güzeli hangi metotların 3 parametreden fazla sayıda parametre aldıklarını kontrol ediyoruz.
                Dolayısıyla statik kodu analiz etme şansımız var.
            */

            // var codeOfFile = await File.ReadAllTextAsync("Utility.cs");

            // // Önce kod dosyasının içeriğin kullanılabilir halde parse ediyoruz
            // var codeTree = CSharpSyntaxTree.ParseText(codeOfFile);
            // // Ardından ağaç yapısının tepesindeki elemanı alıyoruz (XML Root'u almak gibi düşündüm)
            // var rootOfTree = codeTree.GetCompilationUnitRoot();

            // Console.WriteLine($"Kodun kullandığı {rootOfTree.Usings.Count} using tanımı var. Bunlar...");
            // foreach (var u in rootOfTree.Usings)
            //     Console.WriteLine(u.Name);

            // Console.WriteLine("");

            // var ns = (NamespaceDeclarationSyntax)rootOfTree.Members[0];
            // Console.WriteLine($"Root namespace adı {ns.Name}. İçindeki sınıflar...");
            // foreach (ClassDeclarationSyntax c in ns.Members)
            // {
            //     Console.WriteLine($"{c.Identifier} sınıfının {c.Members.Count} üyesi var.");
            // }

            // Console.WriteLine();
            // // parametre sayısı üçten fazla olan metotların listesini alıyoruz
            // var penalties = from m in rootOfTree.DescendantNodes().OfType<MethodDeclarationSyntax>()
            //                 where m.ParameterList.Parameters.Count > 3
            //                 select m;

            // Console.WriteLine("Üçten fazla parametre alan metotların listesi.");
            // foreach (MethodDeclarationSyntax p in penalties)
            // {
            //     // Metodun adını ve parametre sayısını yazdırıyoruz.
            //     Console.WriteLine($"{p.Identifier} metodu {p.ParameterList.Parameters.Count} parametre alıyor.");
            //     // Daha iyi bir yolunu bulmam lazım. Burada tahmini kod satır sayısını bulmaya çalışıyorum.
            //     // Ancak sizin de fark edeceğiniz üzere satır sayıları fazla çıkıyor.
            //     Console.WriteLine($"Yaklaşık {p.GetText().Lines.Count()} satır kod içeriyor.");

            //     // foreach(TextLine l in p.GetText().Lines)
            //     // {
            //     //     Console.WriteLine(l);
            //     // }
            // }

            #endregion Syntax Tree Mevzusu

            #region Code Walker Mevzusu

            /*
                Program kod ağacının tüm node'larında gezmek istediğimiz durumlarda olabilir.
                En sık verilen örnek kodun renklendirilerek HTML'e alınmasıdır. 
                Ancak statik kod analizi vakalarında da ele alınabilir.

                Olay dile özgü bir Walker sınıfı yazmaktan ibarettir. 
                Örneğin C# tarafı için CSharpSyntaxWalker türevli bir sınıf iş görür.

                MethodCollector sınıfını örnek olarak inceleyebiliriz.
            */

            var tree = CSharpSyntaxTree.ParseText(File.ReadAllText("Utility.cs"));
            var walker = new MethodAnalyzer(); //Walker nesnesini örnekledik
            walker.Visit(tree.GetRoot()); // ve kod ağacını ziyaret ederek dallarında dolaşmasını sağladık
            Console.WriteLine($"Kaynak kod dosyasında adı 15 karakterden fazla olan {walker.Methods.Count} metot bulunmuştur");
            foreach (var m in walker.Methods)
            {
                Console.WriteLine(m.Identifier);
            }

            #endregion Code Walker Mevzusu
        }
    }

    /*
        CSharpSyntaxWalker'dan türetilen sınıfla amacımız metodları dolaşmak.

        Override ettiğimiz VisitMethodDeclaration bu nesnenin gezindiği her metod node'u için çalışacaktır.
        Örneğin bu metod içerisinde, yakalanan metodun ismini analiz edebilir ve bir takım ihlalleri tespit edebiliriz.

        Diğer Override edilen VisitClassDeclaration metodunda ise kod ağacının sınıfları yakalanır.
        Şimdilik sadece girilen sınıf adı yazdırılmıştır. Ancak isim ihlaline takılan metodun hangi sınıfta olduğu bilgisini de buradan alabiliriz.
    */
    public class MethodAnalyzer
        : CSharpSyntaxWalker
    {
        public MethodAnalyzer()
        {
            Methods = new List<MethodDeclarationSyntax>();
        }

        // İhlallere göre tespit ettiğimiz metotları topladığımız koleksiyon
        // Class, Struct vb enstrümanlar için de bu tip dışarıya açık koleksiyonlar tercih edilebilir
        public List<MethodDeclarationSyntax> Methods { get; }
        public override void VisitClassDeclaration(ClassDeclarationSyntax classNode)
        {
            Console.WriteLine($"Ziyaret edilen sınıf {classNode.Identifier}");
            base.VisitClassDeclaration(classNode);
        }

        public override void VisitMethodDeclaration(MethodDeclarationSyntax methodNode)
        {
            // Metod adı 15 karakterden uzunsa bir ihlal olduğunu farz edip listeye ekliyoruz
            if (methodNode.Identifier.ToString().Length > 15)
            {
                Methods.Add(methodNode);
                // Burada ilgili metodun parametre sayısını yakalıyoruz. Belki parametre sayısı ihlalleri olan metodların bulunmasını denerken işinize yarar.
                Console.WriteLine($"{methodNode.Identifier} için parametre sayısı {methodNode.ParameterList.Parameters.Count}"); 
            }
            base.VisitMethodDeclaration(methodNode);
        }
    }
}
