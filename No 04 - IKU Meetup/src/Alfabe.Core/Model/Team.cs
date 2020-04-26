using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

namespace Alfabe.Core
{
    public class Team
    {
        [BsonId]
        [BsonRepresentation(BsonType.ObjectId)]
        public string Id { get; set; }

        [BsonElement("region")]
        public string Region { get; set; }

        [BsonElement("name")]
        public string Name { get; set; }

        [BsonElement("abbrev")]
        public string Abbrevation { get; set; }

        [BsonElement("pop")]
        public double Popularity { get; set; }

        [BsonElement("imgURL")]
        public string Logo { get; set; }
    }
}