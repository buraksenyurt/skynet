using System;

namespace GameModel
{
    public class Card
    {
        public int CardID { get; set; }
        public string Name { get; set; }
        public string Description { get; set; }
        public byte Spell { get; set; }
        public byte Attack { get; set; }
        public byte Health { get; set; }
        public Hero Hero { get; set; }
    }   
}