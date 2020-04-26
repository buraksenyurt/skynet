using Alfabe.Core;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Collections.Generic;

namespace Alfabe.Test
{
    [TestClass]
    public class TeamServiceTests
    {
        private TeamService service;

        [TestInitialize]
        public void Setup()
        {
            IMongoDbSettings settings = new MongoDbSettings
            {
                ConnectionString = "mongodb://localhost:27017",
                Database = "NBADatabase",
                Collection = "teams"
            };
            service = new TeamService(settings);
        }

        [TestMethod]
        public void GetAll_Should_Return_Teams()
        {            
            List<Team> teams = service.GetAll();
            Assert.AreNotEqual(0, teams.Count);
        }

        [TestMethod]
        public void GetSingle_Should_Return_Any_Team_By_Id()
        {
            string id = "5ea47de6abcd077fb26e11c1";
            Team team = service.GetSingle(id);
            Assert.AreNotEqual(null, team);
            Assert.AreEqual("Hawks", team.Name);
        }

        [TestMethod]
        public void Create_Should_Insert_New_Team()
        {
            Team team = new Team
            {
                Name = "Celtics",
                Region = "Boston",
                Popularity = 7.5,
                Abbrevation = "BOS",
                Logo = "http://i.cdn.turner.com/nba/nba/media/celtics_logo.gif"
            };
            var inserted=service.Create(team);
            Assert.AreNotEqual(0, inserted.Id);
            Assert.AreEqual(24, inserted.Id.Length);
        }

        [TestMethod]
        public void Delete_Should_Remove_Team()
        {
            var id = "5ea48e492f2a710680116728";
            var deletedCount = service.Delete(id);
            Assert.AreEqual(1, deletedCount);
        }

        [TestMethod]
        public void Update_Should_Change_Team_Info()
        {
            string id = "5ea48e492f2a710680116728";
            Team currentInfo = new Team
            {
                Id = "5ea48e492f2a710680116728",
                Name ="Celtics(Yoncalar)"
            };
            long updatedCount=service.Update(id, currentInfo);
            Assert.AreEqual(1, updatedCount);
        }
    }
}
