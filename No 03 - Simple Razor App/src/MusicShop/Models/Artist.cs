using System.Collections.Generic;

namespace MusicShop.Models
{
    public class Artist{
        public int ID{get;set;}
        public string Name { get; set; }
        public string Biography { get; set; }
        public List<Album> Albums { get; set; }
    }
}