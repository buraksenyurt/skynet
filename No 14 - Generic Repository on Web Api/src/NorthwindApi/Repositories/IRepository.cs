using System;
using System.Collections.Generic;
using System.Threading.Tasks;
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
        Task Create(T entity);
        Task<T> Read(int id);
        Task Update(T entity);
        Task Delete(int id);
        Task<IEnumerable<T>> ReadAll();
    }
}