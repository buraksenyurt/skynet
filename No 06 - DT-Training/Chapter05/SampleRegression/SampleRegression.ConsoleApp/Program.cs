//*****************************************************************************************
//*                                                                                       *
//* This is an auto-generated file by Microsoft ML.NET CLI (Command-Line Interface) tool. *
//*                                                                                       *
//*****************************************************************************************

using System;
using SampleRegression.Model;

namespace SampleRegression.ConsoleApp
{
    class Program
    {
        static void Main(string[] args)
        {
            // Tahminlemede bulunacağımız seyahat için gerekli girdi değerleri
            ModelInput sampleData = new ModelInput()
            {
                Vendor_id = @"CMT", // Taxi firmasının ID bilgisi   
                Rate_code = 1F,
                Passenger_count = 3F, // Yolcu sayısı
                Trip_time_in_secs = 2418F, // Yolculuk süresi
                Trip_distance = 3.6F, // Katedilen mesafe
                Payment_type = @"CRD", //nakit veya kredi kartı ödemesi. Burada kredi kartı
            };

            // Tahminleme sonucu
            var predictionResult = ConsumeModel.Predict(sampleData);

            Console.WriteLine("Girdi bilgilerimiz...");
            Console.WriteLine($"Firma ID: {sampleData.Vendor_id}");
            Console.WriteLine($"Rating: {sampleData.Rate_code}");
            Console.WriteLine($"Yolcu Sayisi: {sampleData.Passenger_count}");
            Console.WriteLine($"Seyahat suresi: {sampleData.Trip_time_in_secs}");
            Console.WriteLine($"Mesafe: {sampleData.Trip_distance}");
            Console.WriteLine($"Odeme Tipi: {sampleData.Payment_type}");
            Console.WriteLine($"\n\nTahmini Odeme Tutari: {predictionResult.Score} $ \n\n");
        }
    }
}
