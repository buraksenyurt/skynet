using MediatR;
using System.Linq;
using Microsoft.EntityFrameworkCore;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Marvil.Model;

namespace Marvil.Handler
{
    /* 
        Davranışımız yine listeleme ama biraz daha farklı.
        Bu sefer gücü belli bir değerin üstünde olan kahramanları döndürüyoruz
    */
    public class GreaterThan
    {
        /*
            CQRS'in Query nesnesi gibi düşünelim.
            Hero tipinden bir liste dönmesi gerektiği ifade ediyor.
            Listenin arama kriterini de property olarak belirliyoruz.
        */
        public class Query : IRequest<List<Hero>>
        {
            public int LevelValue { get; set; }
        }

        public class Handler : IRequestHandler<Query, List<Hero>>
        {
            private MarvilDbContext _context { get; }
            public Handler(MarvilDbContext context)
            {
                _context = context;
            }
            // IRequestHandler arayüzünden gelen aşağıdaki metot Entity Tarafı ile konuşan
            // ve listeyi döndüren operasyonu üstlenmekte
            public async Task<List<Hero>> Handle(Query request, CancellationToken cancellationToken)
            {
                var heroes = await _context.Heroes.Where(h => h.ForceLevel >= request.LevelValue).ToListAsync();
                return heroes;
            }
        }
    }
}