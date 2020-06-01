using System;
using System.Linq;
using System.Collections.Generic;
using Microsoft.EntityFrameworkCore;
using NorthwindApi.Models;
using NorthwindApi.Context;

namespace NorthwindApi.Repositories
{
    /*
    IRepository tipinden belirtilen sözleşmeyi uygulayan asıl tipimiz.
    Çalışma zamanında bu nesne üretildiğinde T görülen her yer verilen Entity
    nesnesi ile çalışacak.
    */
    public class Repository<T>
        : IRepository<T>
        where T : Entity // T türü Entity türevli olmak zorunda. 
    {
        private readonly NorthwindContext _context;
        private DbSet<T> _entity;
        // Entity Framework DbContext türevini Constructor üzerinden içeriye alıyoruz
        public Repository(NorthwindContext context)
        {
            _context = context;
            _entity = context.Set<T>(); // Repository hangi Entity tipi ile çalışacaksa onu yüklüyoruz.
        }
        public void Create(T entity)
        {
            if (entity == null)
            {
                throw new ArgumentNullException("Entity boş gelemez");
            }

            _entity.Add(entity);
            _context.SaveChanges();
        }
        public T Read(int id) => _entity.SingleOrDefault(e => e.Id == id);
        public void Update(T entity)
        {
            throw new NotImplementedException(); // Ödev
        }
        public void Delete(int id)
        {
            T entity = _entity.SingleOrDefault(e => e.Id == id);
            _entity.Remove(entity);
            _context.SaveChanges();
        }
        public IEnumerable<T> ReadAll() => _entity.AsEnumerable();
    }
}