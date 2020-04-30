using Cooking.Entity;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Http;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;

namespace CookingAPI
{
    public class Startup
    {
        public void ConfigureServices(IServiceCollection services)
        {
            services.AddGrpc();

            services.AddDbContext<GeniusRecipesContext>(options =>
                    options.UseNpgsql("host = localhost; port = 5433; database = geniusrecipes; username = postgres; password = P@ssw0rd"));
        }

        public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
        {
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
            }

            app.UseRouting();

            app.UseEndpoints(endpoints =>
            {
                endpoints.MapGrpcService<CookService>();

                endpoints.MapGet("/", async context =>
                {
                    await context.Response.WriteAsync("Çok basit bir gRPC uygulaması pratiği.");
                });
            });
        }
    }
}
