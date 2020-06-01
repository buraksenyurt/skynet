using System;
using System.Collections.Generic;
using NorthwindApi.Models;

namespace NorthwindApi.Repositories
{
    /*
    Temel CRUD operasyonlarını tanımlayan generic interface tipimiz.
    Ekstradan tüm entity içeriğini de döndüren ReadAll operasyonunu da tanımlıyor.
    */
    public interface IRepository<T>
        where T : Entity //T tipi Entity türünden olmalı. Şu durumda Category ve Product olabilir
    {
        void Create(T entity);
        T Read(int id);
        void Update(T entity);
        void Delete(int id);
        IEnumerable<T> ReadAll();
    }
}