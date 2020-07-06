using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using System.IO; // Eklendi
using Microsoft.EntityFrameworkCore; // Eklendi
using NorthwindLib; // Eklendi

namespace GameWorldWeb
{
    public class Startup
    {
        // This method gets called by the runtime. Use this method to add services to the container.
        // For more information on how to configure your application, visit https://go.microsoft.com/fwlink/?LinkID=398940
        public void ConfigureServices(IServiceCollection services)
        {
            // Razor sayfa desteği için eklendi
            services.AddRazorPages();

            // Entity Framework ve SQLite kullanacağız. Fiziki adres sonrası
            string dbPath = Path.Combine("..", "NorthwindGameCatalog.db");
            // EF servisini de register ediyoruz.
            services.AddDbContext<Northwind>(options => options.UseSqlite($"Data Source={dbPath}", b => b.MigrationsAssembly("GameWorldWeb")));
        }

        // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
        public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
        {
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
            }
            else //Eklendi
            {
                app.UseHsts(); //HTTP Strict Transport Security(HSTS)
                // Tüm iletişimin HTTPS üzerinden yapılmasına zorlar
            }

            app.UseRouting();

            // Eklendi
            // HTTP Taleplerini HTTPS'e yönlendirir   
            // F12 ile izlendiğinde http://localhost:5000/ sonrası 307 Temporary Redirect ile 5001'e gidildiği gözlemlenmelidir.         
            app.UseHttpsRedirection();

            // Alttaki iki satır index.html, default.html gibi varsayılan sayfaların desteği için eklendi
            // Index sayfasının çalışması için alttaki UseEndpoints kısmı kapatıldı
            app.UseDefaultFiles();
            app.UseStaticFiles();

            app.UseEndpoints(endpoints =>
            {
                //     endpoints.MapGet("/", async context =>
                //     {
                //         await context.Response.WriteAsync("Hello World!");
                //     });
                endpoints.MapRazorPages(); // Razor sayfalarını eşleştirmek için açıldı
            });
        }
    }
}
