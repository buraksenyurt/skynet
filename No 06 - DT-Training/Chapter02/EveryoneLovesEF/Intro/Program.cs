using System;
using System.Linq;
using System.Collections.Generic;
using Intro.Model;
// Aşağıdakiler EF Log mekanizmasını enjekte etmek için eklendiler.
using Microsoft.EntityFrameworkCore.Infrastructure;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace Intro
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Nickname ?");
            string nickName = Console.ReadLine();
            Console.WriteLine("Level ?");
            short level = Convert.ToInt16(Console.ReadLine());
            using (GameContext context = new GameContext())
            {
                // Add işlemlerinde log
                var logFactory = context.GetService<ILoggerFactory>();
                logFactory.AddProvider(new ConsoleLoggerProvider());

                Player player = new Player
                {
                    Nickname = nickName,
                    Level = level
                };

                context.Players.Add(player);
                context.SaveChanges();

                var allPlayers = from p in context.Players
                                 orderby p.Nickname
                                 select p;
                foreach (var p in allPlayers)
                    Console.WriteLine($"{p.PlayerId} {p.Nickname} {p.Level}");
            }
        }
    }
}