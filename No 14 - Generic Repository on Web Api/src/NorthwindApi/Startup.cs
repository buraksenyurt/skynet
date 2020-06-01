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
using Microsoft.EntityFrameworkCore;
using NorthwindApi.Context;
using NorthwindApi.Repositories;

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
            // SQLite kullanacağımızı belirtiyoruz.             
            // Connection String bilgisini appsettings.json'dan alıyoruz.
            services.AddDbContext<NorthwindContext>(opt => opt.UseSqlite(Configuration.GetConnectionString("NorthwinContext")));
            // NorthwindContext repository sınıflarına DI tarafından bu bildirim sonrası otomatik geçecek (Constructor'lar üzerinden)
            services.AddScoped<NorthwindContext>();
            // Benzer şekilde IRepository kullanılan yerlere de Repository türleri otomatik olarak geçecek.
            services.AddScoped(typeof(IRepository<>), typeof(Repository<>));
            // Aynı şekilde IProductRepository olan yerlere de DI mekanizması ProductRepository örneklerini bağlayacak.
            services.AddScoped(typeof(IProductRepository), typeof(ProductRepository));
            services.AddControllers();
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
