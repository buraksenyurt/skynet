using System;
using System.Linq;
using System.Collections.Generic;
using Microsoft.EntityFrameworkCore;
using NorthwindApi.Models;
using NorthwindApi.Context;
using System.Threading.Tasks;

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
        protected readonly NorthwindContext _context;
        private DbSet<T> _entity;
        // Entity Framework DbContext türevini Constructor üzerinden içeriye alıyoruz
        public Repository(NorthwindContext context)
        {
            _context = context;
            _entity = context.Set<T>(); // Repository hangi Entity tipi ile çalışacaksa onu yüklüyoruz.
        }
        public async Task Create(T entity)
        {
            if (entity == null)
            {
                throw new ArgumentNullException("Entity boş gelemez");
            }

            await _entity.AddAsync(entity);
            _context.SaveChanges();
        }
        public async Task<T> Read(int id) => await _entity.SingleOrDefaultAsync(e => e.Id == id);
        public Task Update(T entity)
        {
            throw new NotImplementedException(); // Ödev
        }
        public async Task Delete(int id)
        {
            T entity = await _entity.SingleOrDefaultAsync(e => e.Id == id);
            _entity.Remove(entity);
            await _context.SaveChangesAsync();
        }
        public async Task<IEnumerable<T>> ReadAll() => await _entity.ToListAsync();
    }
}