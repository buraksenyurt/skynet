using System.ComponentModel.DataAnnotations; // Veri doğrulama işlemleri için eklendi.
using System.Collections.Generic;

namespace GamerMVC.Models
{
    public class CompanyGameModel
    {
        [Required(ErrorMessage="Dostum firmanın bir adı olmalı")]
        [StringLength(50,ErrorMessage="Firma adı 50 karakterle sınırlı :(")]
        public string Name { get; set; }
        [Required(ErrorMessage="Firma hakkında bir şeyler girmelisin")]
        [StringLength(250,ErrorMessage="En fazla 250 karaktere izin verebilirim")]
        public string Description { get; set; }
        [Required(ErrorMessage="Dostum en azından bir oyunlarının adını yazmalısın")]
        [StringLength(50,ErrorMessage="Oyun adı 50 karakterle sınırlı tutabilir misin?")]
        public string GameTitle { get; set; }
        [Range(1978,1999,ErrorMessage="Sadece 1978 ile 1999 arasındaki oyunları kabul ediyoruz")]
        public short GameYear { get; set; }=1978;
        public short GamePopularity { get; set; }
        public bool GameIsContinued { get; set; }
        public bool HasErrors { get; set; } // Modelin hatalar içerip içermediği bilgisini tutarız
        public IEnumerable<string> ValidationErrors { get; set; } //Doğrulama kuralı hatalarını burada depolayabiliriz

    }
}