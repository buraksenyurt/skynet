using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace Intro.Model
{
    public class Player
    {
        public int PlayerId { get; set; }
        [StringLength(12)]
        [Required]
        [Column("nick_name")]
        public string Nickname { get; set; }
        [Required]
        [Column("current_level")]
        public short Level { get; set; }
    }
}