using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
// Ocelot için gerekli bildirimler
using Ocelot.DependencyInjection;
using Ocelot.Middleware;
using System.Net.Http;
using System.Threading;

namespace Bosphorus
{
    public class Program
    {
        public static void Main(string[] args)
        {
            CreateHostBuilder(args).Build().Run();
        }

        public static IHostBuilder CreateHostBuilder(string[] args) =>
            Host.CreateDefaultBuilder(args).ConfigureServices(services =>
            {
                services
                    .AddOcelot() // Ocelot'u bildirdik
                    .AddDelegatingHandler<RequestInspector>(); // HttpClient isteklerinde araya girecek delegeyi bildirdik
            }).ConfigureAppConfiguration((host, config) =>
            {
                config.AddJsonFile("ocelot.json"); // Ocelot ayarlarının alınacağı konfigurasyon dosyasını belirttik
            })
            .ConfigureWebHostDefaults(webBuilder =>
            {
                webBuilder.UseStartup<Startup>().Configure(async app => await app.UseOcelot());
            });
    }

    /*
        Aşağıdaki sınıfın yardımıyla Ocelot'taki belirttiğimiz bir Route'a gelen HTTP istekleri işlenmeden önce araya girebiliriz.
        Request içeriğine bakıp akışı değiştirebiliriz.
        Bu temsilci sınıfını kullanacağımızı yukarıdaki AddDelegatingHandler metodunda belirttik.
        Ayrıca ocelot.json içerisinde, örnek olması açısından /eagames/player/{id} adresine gelen taleplerde araya gireceğimizi belirttik.
    */
    public class RequestInspector : DelegatingHandler
    {
        protected override async Task<HttpResponseMessage> SendAsync(HttpRequestMessage request, CancellationToken cancellationToken)
        {
            Console.WriteLine($"\nDevam etmeden önce şu gelen Request içeriğini bir inceyelim\n{request.ToString()}\n");
            return await base.SendAsync(request, cancellationToken);
        }
    }
}
