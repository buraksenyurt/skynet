namespace GamerMVC.Models
{
    public class CompanyGameModel
    {
        public string Name { get; set; }
        public string Description { get; set; }
        public string GameTitle { get; set; }
        public short GameYear { get; set; }
        public short GamePopularity { get; set; }
        public bool GameIsContinued { get; set; }
    }
}