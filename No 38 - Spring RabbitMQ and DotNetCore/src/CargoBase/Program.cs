using System;
using RabbitMQ.Client;
using System.Text;
using Newtonsoft.Json;

namespace CargoBase
{
    class Program
    {
        static void Main(string[] args)
        {

            Random _random = new Random();
            // Factory nesnesi üstünden RabbitMQ'ya bir bağlantı açacağız
            var factory = new ConnectionFactory() { HostName = "localhost" };
            using (var connection = factory.CreateConnection())
            {
                // sonrasında kanal tanımlama ve mesaj gönderme işi için gerekli nesneleri üreteceğiz
                using (var channel = connection.CreateModel())
                {
                    string queueName = "package-state-action";
                    // Bir kuyruk tanımladık. Kargonun durum değişikliği ile alakalı bir kuyruk gibi düşünelim
                    channel.QueueDeclare(queue: queueName,
                                         durable: false,
                                         exclusive: false,
                                         autoDelete: false,
                                         arguments: null);

                    // Kuyruğu JSON olarak serileştirilmiş bir nesne koyalım. Kobay nesnemiz Package türünden bir örnek.
                    var package = JsonConvert.SerializeObject(
                            new Package
                            {
                                SerialNo = _random.Next(1, 1000),
                                State = "Ready",
                                Weight = _random.NextDouble()*100,
                                Time = DateTime.Now.ToString()
                            });
                    // nesne içeriğini kanala yazmak için Byte[] dizisine çeviriyoruz
                    var body = Encoding.UTF8.GetBytes(package);
                    Console.WriteLine($"{package} içeriği gönderilecek");

                    // routingKey bilgisi ile de yukarıda tanımlanan kanala mesajımızı bırakalım
                    channel.BasicPublish(exchange: "",
                                         routingKey: queueName,
                                         basicProperties: null,
                                         body: body);
                }

                Console.WriteLine("Kargo için durum bilgisi yayınlandı. Çıkmak için bir tuşa basınız");
                Console.ReadLine();
            }
        }
    }

    public class Package
    {
        public int SerialNo { get; set; }
        public string State { get; set; }
        public double Weight { get; set; }
        public string Time { get; set; }
    }
}