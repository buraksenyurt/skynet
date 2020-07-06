using System.Collections.Generic;

namespace NorthwindLib
{
    public class Game{
        public int GameID { get; set; }
        public string Title { get; set; }
        public int? CompanyID { get; set; }
        public short? Year { get; set; }=1978;
        public short? Popuplarity { get; set; }=0;
        public bool Discontinued { get; set; }=false;
        public Company Company { get; set; }
    }
}