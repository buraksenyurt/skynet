using System;
using System.Linq;
using System.Collections.Generic;
using Intro.Model;

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
            AddPlayer(nickName, level);
            WritePlayers();
        }

        static void AddPlayer(string nickName, short level)
        {
            using (GameContext context = new GameContext())
            {
                Player player = new Player
                {
                    Nickname = nickName,
                    Level = level
                };

                context.Players.Add(player);
                context.SaveChanges();
            }
        }

        static void WritePlayers()
        {
            using (GameContext context = new GameContext())
            {
                var allPlayers = from p in context.Players
                                 orderby p.Nickname
                                 select p;
                foreach (var p in allPlayers)
                    Console.WriteLine($"{p.PlayerId} {p.Nickname} {p.Level}");
            }
        }
    }
}
