using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.Extensions.Logging;
// ML kütüphane ve model proje bildirimleri
using Microsoft.Extensions.ML;
using SampleClassification.Model;

namespace ClientApp.Pages
{
    public class IndexModel : PageModel
    {
        private readonly ILogger<IndexModel> _logger;
        private readonly PredictionEnginePool<ModelInput, ModelOutput> _predictionEnginePool;
        public string Result;
        [BindProperty]
        public string IncomingComment { get; set; }

        /*
            Startup tarafında eklediğimiz tahminleme motorunu, constructor üzerinden bu modele alıyoruz.
        */
        public IndexModel(ILogger<IndexModel> logger,PredictionEnginePool<ModelInput, ModelOutput> predictionEnginePool)
        {
            _predictionEnginePool = predictionEnginePool;
            _logger = logger;
        }
        public void OnPost(string Comment)
        {
            if (String.IsNullOrEmpty(Comment))
            {
                Result = "Yorumsuz";
                return;
            }

            IncomingComment=Comment;

            _logger.LogInformation($"Gelen içerik {Comment}");

            /*
                Gelen yorumu tahminleme motoruna gönderiyor ve bir sonuç alıyoruz.
                Bu sonucun 1 ve 0 olma durumuna göre de gelen yorumun zararlı veya zararsız olduğu hakkındaki sonucu modeldeki özelliğie bağlıyoruz.
            */
            var input = new ModelInput { SentimentText = Comment };
            var prediction = _predictionEnginePool.Predict(input);
            _logger.LogInformation($"Tahmin sonucu {prediction}");
            var sentiment = prediction.Prediction=="1" ? "Zararlı" : "Zararsız";
            Result = sentiment;
        }
    }
}
