using System;
using System.ComponentModel;
using System.ComponentModel.DataAnnotations;

namespace MusicShop.Models
{
    public class Album
    {
        public int ID { get; set; }
        [StringLength(100, MinimumLength = 3)]
        [DisplayName("Albümün Adı")]
        [Required]
        public string Title { get; set; }
        [DisplayName("Söyleyen")]
        public int ArtistID { get; set; }
        public Artist Artist { get; set; }
        //[DataType(DataType.Upload)]  //Diğer veri türleri için -> https://docs.microsoft.com/en-us/dotnet/api/system.componentmodel.dataannotations.datatype?view=netcore-3.1
        public byte[] Cover { get; set; }
        [Range(1950,2020)]        
        [DisplayName("Çıkış Yılı")]
        [Required]
        public short ReleaseYear { get; set; }
    }
}