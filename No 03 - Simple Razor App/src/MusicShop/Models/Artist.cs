using System.Collections.Generic;
using System.ComponentModel;
using System.ComponentModel.DataAnnotations;

namespace MusicShop.Models
{
    public class Artist{
        public int ID{get;set;}
        [StringLength(60, MinimumLength = 3)]
        [DisplayName("Sanatçı/Grup Adı")]
        [Required]
        public string Name { get; set; }
        [StringLength(1000, MinimumLength = 100)]
        [DisplayName("Biyografi")]
        [Required]
        public string Biography { get; set; }
        public List<Album> Albums { get; set; }
    }
}