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
        Task CreateAsync(T entity);
        Task<T> ReadAsync(int id);
        Task UpdateAsync(T entity);
        Task<bool> DeleteAsync(int id);
        Task<IEnumerable<T>> ReadAllAsync();
    }
}