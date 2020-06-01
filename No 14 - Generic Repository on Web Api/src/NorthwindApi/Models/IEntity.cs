using System;

namespace NorthwindApi.Models
{
    /*
    Entity sınıfı için bir sözleşme tanımlıyoruz.
    User tipi de isteğe bağlı olarak üretilip çalışmaya eklenebilir.
    */
    public interface IEntity
    {
        int Id { get; set; }
        DateTime CreateDate { get; set; }
        //User CreateUser { get; set; }
        DateTime? UpdateDate { get; set; }        
        // User UpdateUser { get; set; }
    }
}