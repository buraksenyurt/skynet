using System;

namespace MusicShop.Models
{
    public class Album{
        public int ID{get;set;}
        public string Title { get; set; }
        public int ArtistID { get; set; }
        public Artist Artist { get; set; }
        public byte[] Cover { get; set; }
        public short ReleaseYear { get; set; }
    }
}