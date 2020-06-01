using System;

namespace NorthwindApi.Models
{
    /*
    Asıl Entity tiplerinin(Category, Product, User gibi) türetileceği sınıf.
    */
    public class Entity
        :IEntity
    {
        public int Id { get; set; }
        public DateTime CreateDate { get; set; } = DateTime.Now;
        //public User CreateUser { get; set; }
        public DateTime? UpdateDate { get; set; }        
        //public User UpdateUser { get; set; }
    }
}