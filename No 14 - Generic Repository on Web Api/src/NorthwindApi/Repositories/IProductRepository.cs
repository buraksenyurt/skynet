using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using NorthwindApi.Models;
using System.Collections.Generic;

namespace NorthwindApi.Repositories
{
    /*
    Bazen bir Entity türü için repository'nin fonksiyon bazında farklılaşması gerekebilir.
    Bu durumda yeni bir sözleşme aşağıdaki gibi oluşturulup uygulanabilir.
    */
    public interface IProductRepository
        : IRepository<Product>
    {
        Task<List<Product>> GetLessThanStockValueAsync(int stockLevel);
    }
}