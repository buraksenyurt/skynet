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
using Marvil.Model;
using Marvil.Handler;
using MediatR;

namespace Marvil
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
            // DbContext nesnesini middelware'e ekliyoruz ve appsettings dosyasındaki bağlantı bilgisi doğrultusunda Sqlite kullanacak şekilde ayağa kaldırıyoruz.
            services.AddDbContext<MarvilDbContext>(opt =>
            {
                opt.UseSqlite(Configuration.GetConnectionString("MarvilDbConnection"));
            });

            // Mediator nesnesini servis olarak çalışma zamanına ekliyoruz            
            services.AddMediatR(typeof(List.Handler).Assembly);

            services.AddControllers();
        }

        // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
        public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
        {
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
            }

            //app.UseHttpsRedirection();

            app.UseRouting();

            //app.UseAuthorization();

            app.UseEndpoints(endpoints =>
            {
                endpoints.MapControllers();
            });
        }
    }
}
