using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Caching.Distributed;
using System.Text;

namespace WeatherCollector
{
    public class Worker : BackgroundService
    {
        private readonly ILogger<Worker> _logger;
        private readonly IDistributedCache _distributedCache; //Redis için gerekli
        private Task _executingTask;
        private CancellationTokenSource _cts;

        // Loglama ve Redis için gerekli nesneleri constructor'dan içeriye enjekte ediyoruz
        public Worker(ILogger<Worker> logger, IDistributedCache distributedCache) 
        {
            _logger = logger;
            _distributedCache = distributedCache;
        }

        // Servis başladığında devreye giren metot. Override etmek zorunda değiliz
        public override Task StartAsync(CancellationToken cancellationToken)
        {
            _logger.LogWarning($"Weather Collector service started at {DateTimeOffset.Now}");

            _cts = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
            _executingTask = ExecuteAsync(_cts.Token);

            return _executingTask.IsCompleted ? _executingTask : Task.CompletedTask;
        }

        // Arka plan görevinin başladığı metot
        protected override async Task ExecuteAsync(CancellationToken stoppingToken)
        {
            var options = new DistributedCacheEntryOptions()
                       .SetSlidingExpiration(TimeSpan.FromMinutes(5))
                       .SetAbsoluteExpiration(DateTime.Now.AddHours(1));

            while (!stoppingToken.IsCancellationRequested) // Eğer bir iptal talebi gelmediyse
            {
                _logger.LogInformation($"Looking for weather informations at: {DateTimeOffset.Now}");

                // Normalde hava durumu verisi harici bir servisten geliyor olmalı.
                // Burada tamamen sembolik bir JSON içeriği söz konusu
                var temprature = "[{\"city\":\"İstanbul\",\"value\":\"39\"},{\"city\":\"Ankara\",\"value\":\"34\"}]";
                var redisValue = Encoding.UTF8.GetBytes(temprature);

                // veriyi Redis Cache'e alıyoruz. Farklı bir veritabanı da kullanılabilir
                await _distributedCache.SetAsync($"State_{DateTime.Now.Day}_{DateTime.Now.ToString("hh_mm_ss")}", redisValue, options);

                // Arka plan görevi bu eğitim örneği özelinde 3 dakikada bir işleyecek
                await Task.Delay(3 * 60 * 1000, stoppingToken);
            }
        }

        // Servis durdurulduğunda override edilmişse devreye giren metot
        public override Task StopAsync(CancellationToken cancellationToken)
        {
            if (_executingTask == null)
            {
                return Task.CompletedTask;
            }

            _logger.LogWarning($"Weather Collector stopping at: {DateTimeOffset.Now}");
            _cts.Cancel();
            Task.WhenAny(_executingTask, Task.Delay(-1, cancellationToken)).ConfigureAwait(true);
            cancellationToken.ThrowIfCancellationRequested();
            _logger.LogWarning($"Weather Collector stopped at: {DateTimeOffset.Now}");

            return Task.CompletedTask;
        }
    }
}
