using NorthwindLib;
using System.Collections.Generic;
using System.Threading.Tasks;
using System.Linq;
using System;
using System.Collections.Concurrent;
using Microsoft.EntityFrameworkCore.ChangeTracking;

namespace GameWorldApi.Repository
{
    /*
    Company entity'si için tasarlanmış temel CRUD operasyonlarını uygulayan sınıfımız
    */
    public class CompanyRepository
        : ICompanyRepository
    {
        private Northwind _db;
        // Bu örnek özelinde Cache'leyeceğimiz firma bilgilerini tutmak için thread-safe bir dictionary kullanıyoruz.
        private static ConcurrentDictionary<int, Company> _companiesOnCache;
        public CompanyRepository(Northwind db)
        {
            _db = db;

            /*
            Firma bilgilerinin ön belleğe alınması operasyonu.
            */
            if (_companiesOnCache == null)
            {
                _companiesOnCache = new ConcurrentDictionary<int, Company>(
                    _db.Companies.ToDictionary(c => c.CompanyID)
                );
            }
        }
        // Tüm ürünleri listeleyen metodumuz
        public Task<IEnumerable<Company>> GetAllAsync()
        {
            // Cache üzerinde duran firma bilgilerini döndürüyoruz
            return Task.Run<IEnumerable<Company>>(() => _companiesOnCache.Values);
        }
        // CompanyID bilgisine göre tek firma döndüren metodumuz
        public Task<Company> GetAsync(int id)
        {
            return Task.Run<Company>(() =>
            {
                // Aranan firma bilgisini Cache'ten getiriyoruz
                _companiesOnCache.TryGetValue(id, out Company founded);
                return founded;
            });
        }
        //Yeni bir oyun firması eklerken kullanacağımız metot
        public async Task<Company> CreateCompanyAsync(Company company)
        {
            await _db.Companies.AddAsync(company); // Koleksiyona ekledik
            int added = await _db.SaveChangesAsync(); //SQL insert ifadesini çalıştırdık
            if (added == 1) // Kayıt oluştuysa
            {
                // Eğer dictionary içinde yoksa ekleyecek ve cache yenilenecek
                return _companiesOnCache.AddOrUpdate(company.CompanyID, company, UpdateCompanyOnCache);
            }
            else
            {
                return null;
            }
        }
        // Firma bilgilerini güncelleyen metodumuz
        public async Task<Company> UpdateAsync(int id, Company company)
        {
            // Önce Entity koleksiyonunda güncelleme yapıyoruz
            _db.Companies.Update(company);
            var updated = await _db.SaveChangesAsync(); // Update sorgusunu gönderiyoruz
            if (updated == 1) // Kayıt güncellenmişse veritabanının cevabı 1 olacaktır
            {
                return UpdateCompanyOnCache(id, company); // Bu durumda Cache üzerinde duran Company nesnesini de güncelliyoruz
            }
            else
            {
                return null;
            }
        }
        //CompanyID değerinden bir firmayı silmemize yarayan metodumuz
        public async Task<bool?> DeleteAsync(int id)
        {
            var founded = _db.Companies.Find(id); // Önce silinmek istenen Company nesnesini bulalım
            _db.Companies.Remove(founded); // Entity koleksiyonundan çıkartalım
            var deletedCount = await _db.SaveChangesAsync(); //Delete sorgusunu gönderelim
            if (deletedCount == 1) //Eğer kayıt silinmişse
            {
                return _companiesOnCache.TryRemove(id, out founded); // Cache'te duran koleksiyondan da çıkartalım
            }
            else
            {
                return null;
            }
        }

        // Cache tazeleme metodumuz
        private Company UpdateCompanyOnCache(int id, Company company)
        {
            // id'ye ait Value değerini oldCompany adlı değişkene almaya çalışıyoruz.
            if (_companiesOnCache.TryGetValue(id, out Company oldCompany))
            {
                // Aldıysak buradaki Company nesnesini güncellemeyi deniyoruz
                if (_companiesOnCache.TryUpdate(id, company, oldCompany))
                {
                    return company;
                }
            }
            return null;
        }
    }
}