using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.HttpsPolicy;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
/*
    Postgresql ve SqlKata için gerekli namespace bildirimleri
*/
using SqlKata;
using SqlKata.Compilers;
using SqlKata.Execution;
using Npgsql; 

namespace NorthwindApi
{
    public class Startup
    {
        public Startup(IConfiguration configuration)
        {
            Configuration = configuration;
        }

        public IConfiguration Configuration { get; }

        // This method gets called by the runtime. Use this method to add services to the container.
        public void ConfigureServices(IServiceCollection services)
        {
            services.AddControllers();

            /*
                QueryFactory sınıfını burada kayıt edip controller'lara constructor üzerinde enjekte ederek kullandırtabiliriz.
                Oluştururken Postgresql bağlantı bilgisini veriyoruz.
                Ayrıca sorgular için gerekli derleyici nesnesi de üretiliyor
            */
            services.AddScoped(factory =>
            {
                return new QueryFactory
                {
                    Compiler = new PostgresCompiler(),
                    Connection = new NpgsqlConnection("Host=localhost;Username=username;Password=pass;Database=db name")
                };
            });
        }


        // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
        public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
        {
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
            }

            app.UseHttpsRedirection();

            app.UseRouting();

            app.UseAuthorization();

            app.UseEndpoints(endpoints =>
            {
                endpoints.MapControllers();
            });
        }
    }
}
