using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Identity.UI;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.HttpsPolicy;
using Microsoft.EntityFrameworkCore;
using GamerMVC.Data;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using System.IO; //Eklendi
using NorthwindLib; //Eklendi
using System.Net.Http.Headers; // Eklendi (Web API kullanımı için)

namespace GamerMVC
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
            // NorthwindGameCatalog isimli SQLite veritabanını kullanabilmek için
            // path ve DbContext ayarlamaları eklendi
            string dbPath = Path.Combine("..", "NorthwindGameCatalog.db");
            services.AddDbContext<Northwind>(options => options.UseSqlite($"Data Source={dbPath}"));

            services.AddDbContext<ApplicationDbContext>(options =>
                options.UseSqlite(
                    Configuration.GetConnectionString("DefaultConnection")));
            services.AddDefaultIdentity<IdentityUser>(options => options.SignIn.RequireConfirmedAccount = true)
                .AddEntityFrameworkStores<ApplicationDbContext>();
            services.AddControllersWithViews();
            services.AddRazorPages();

            /*
                Aşağıdaki kısım GameWorldAPI isimli Web API projesini bu MVC uygulamasında kullanabilmek için eklenmiştir.
                Böylece bir Controller'a IHttpClientFactory üzerinden Web API servisini tüketecek nesneyi aktarabiliriz. 
            */
            services.AddHttpClient(name: "GameWorldService", configureClient: options =>
            {
                options.BaseAddress = new Uri("https://localhost:5551"); // Kullanacağımız web api'nin adresini tanımladık
                options.DefaultRequestHeaders.Accept.Add(new MediaTypeWithQualityHeaderValue("application/json", 1.0)); //Varsayılan olarak Request Header içeriği JSON 1.0 standardında olacak
            });
        }
        // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
        public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
        {
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
                app.UseDatabaseErrorPage();
            }
            else
            {
                app.UseExceptionHandler("/Home/Error");
                // The default HSTS value is 30 days. You may want to change this for production scenarios, see https://aka.ms/aspnetcore-hsts.
                app.UseHsts();
            }
            app.UseHttpsRedirection();
            app.UseStaticFiles();

            app.UseRouting();

            app.UseAuthentication();
            app.UseAuthorization();

            app.UseEndpoints(endpoints =>
            {
                endpoints.MapControllerRoute(
                    name: "default",
                    pattern: "{controller=Home}/{action=Index}/{id?}");
                endpoints.MapRazorPages();
            });
        }
    }
}
