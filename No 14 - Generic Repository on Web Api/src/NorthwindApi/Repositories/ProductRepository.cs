using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using NorthwindApi.Models;
using NorthwindApi.Context;
using System.Linq;
using System.Collections.Generic;

namespace NorthwindApi.Repositories
{
    public class ProductRepository
        : Repository<Product>, IProductRepository
    {
        public ProductRepository(NorthwindContext context)
            : base(context)
        {
        }
        public async Task<List<Product>> GetLessThanStockValue(int stockLevel)
        {
            // _context nesnesi protected işaretlendiği için base sınıftan erişilebilir durumda
            return await _context.Products.Where(p => p.UnitsInStock <= stockLevel).ToListAsync();
        }
    }
}