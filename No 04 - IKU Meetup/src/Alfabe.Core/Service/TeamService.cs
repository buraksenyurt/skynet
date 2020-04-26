using MongoDB.Driver;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Alfabe.Core
{
    public class TeamService
    {
        private readonly IMongoCollection<Team> _teams;

        public TeamService(IMongoDbSettings settings)
        {
            MongoClient client = new MongoClient(settings.ConnectionString);
            var db = client.GetDatabase(settings.Database);
            _teams = db.GetCollection<Team>(settings.Collection);
        }

        public List<Team> GetAll() => _teams.Find(t => true).ToList();

        public Team GetSingle(string id) => _teams.Find(t => t.Id == id).FirstOrDefault();

        public Team Create(Team team)
        {
            //TODO@Burak[DEVELOPMENT] Should check properties
            _teams.InsertOne(team);
            return team;
        }

        public long Delete(string id) => _teams.DeleteOne(t => t.Id == id).DeletedCount;

        public long Update(string id, Team currentInfo) => _teams.ReplaceOne(t => t.Id == id, currentInfo).ModifiedCount;
    }
}