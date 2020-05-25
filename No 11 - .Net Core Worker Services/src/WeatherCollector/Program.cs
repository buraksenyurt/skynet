using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Options;

namespace WeatherCollector  
{
    public class Program
    {
        public static void Main(string[] args)
        {
            CreateHostBuilder(args).Build().Run();
        }

        public static IHostBuilder CreateHostBuilder(string[] args) =>
            Host.CreateDefaultBuilder(args)
                .UseSystemd()
                .ConfigureServices((hostContext, services) =>
                {
                    services.AddHostedService<Worker>();
                    // Redis servisini middleware'e ekledik. Artık Worker sınıfının yapıcı metodundan içine enjekte edilebilir
                    services.AddDistributedRedisCache(action =>
                    {
                        action.Configuration = "localhost:6379";
                    });
                });
    }
}
