//*****************************************************************************************
//*                                                                                       *
//* This is an auto-generated file by Microsoft ML.NET CLI (Command-Line Interface) tool. *
//*                                                                                       *
//*****************************************************************************************

using System;
using SampleClassification.Model;

namespace SampleClassification.ConsoleApp
{
    class Program
    {
        static void Main(string[] args)
        {
            // Modelimizi eğittik. Artık ona örnek yorumlar yollayıp bunun zararlı olup olmadığını tahmin etmesini isteyebiliriz.
            var input = new ModelInput();
            input.SentimentText = "It was a very disgusting article, man.";

            // Tahminlemeyi yaptırıyoruz
            ModelOutput result = ConsumeModel.Predict(input);
            Console.WriteLine($"Girilen Yorum: '{input.SentimentText}'\nZehirli mi?: {result.Prediction}");

            input.SentimentText="Great narration. I took great advantage. Thank you.";
            result=ConsumeModel.Predict(input);
            Console.WriteLine($"Girilen Yorum: '{input.SentimentText}'\nZehirli mi?: {result.Prediction}");
        }
    }
}
