using MediatR;
using Microsoft.EntityFrameworkCore;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Marvil.Model;

namespace Marvil.Handler
{
    /* 
        Davranışımız listeleme. List isimli sınıfla ifade edeceğiz.
        Listelemeye karşılık gelen Query ve Handler sınıflar da bu sınıfın içerisinde tanımlanıyorlar
        IRequest sınıfı MediatR paketiyle gelmekte
    */
    public class List
    {
        // CQRS'in Query nesnesi gibi düşünelim.
        // Hero tipinden bir liste dönmesi gerektiği ifade ediyor
        public class Query : IRequest<List<Hero>> { }

        /*
            Listeleme işini üstlenen Handler sınıfı
            Uyguladığı arayüze göre ilk parametre ile hangi Query nesnesini kullanacağı 
            ikinci parametre ile de Handler'ın geriye ne döndüreceği belirtiliyor
            HeroesController sınıfında bu Handler sınıfının nasıl kullanıldığına dikkat edelim
        */
        public class Handler : IRequestHandler<Query, List<Hero>>
        {
            // Entity Context nesnesi Constructor üstünde enjekte ediliyor
            private MarvilDbContext _context { get; }
            public Handler(MarvilDbContext context)
            {
                _context = context;
            }
            // IRequestHandler arayüzünden gelen aşağıdaki metot Entity Tarafı ile konuşan
            // ve listeyi döndüren operasyonu üstlenmekte
            public async Task<List<Hero>> Handle(Query request, CancellationToken cancellationToken)
            {
                var heroes = await _context.Heroes.ToListAsync();
                return heroes;
            }
        }
    }
}