using NorthwindLib;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace GameWorldApi.Repository
{
    /*
    Company entity'si için tasarlanmış temel CRUD operasyonlarını içeren sözleşmemiz.
    Sözleşmenin uyarlaması CompanyRepository sınıfı içerisindedir.
    */
    public interface ICompanyRepository
    {
        Task<IEnumerable<Company>> GetAllAsync();
        Task<Company> GetAsync(int id);
        Task<Company> CreateCompanyAsync(Company company);
        Task<Company> UpdateAsync(int id,Company company);
        Task<bool?> DeleteAsync(int id);
    }
}