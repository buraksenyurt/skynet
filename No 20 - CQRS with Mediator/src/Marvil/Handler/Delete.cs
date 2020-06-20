using System;
using MediatR;
using Microsoft.EntityFrameworkCore;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Marvil.Model;

namespace Marvil.Handler
{
    /*
        Operasyon silme.
        Create operasyonu gibi geriye veri döndürmez.
        Command ve Handler buna göre tasarlanır.
    */
    public class Delete
    {

        // Silinmek istenen verinin Name bilgisi yeterli. Command sınıfını buna göre tasarlıyoruz.
        // Geriye veri döndürmediğinden generic olmayan IRequest'i kullandık
        public class Command : IRequest
        {
            public string Name { get; set; }
        }

        // Silme operasyonunu ele alan Handler sınıfı
        public class Handler : IRequestHandler<Command>
        {
            private readonly MarvilDbContext _context;

            public Handler(MarvilDbContext context)
            {
                _context = context;
            }

            public async Task<Unit> Handle(Command request, CancellationToken cancellationToken)
            {
                // Önce kahramanı bulalaım
                var hero = await _context.Heroes.FirstOrDefaultAsync(h => h.Name == request.Name);
                if (hero == null) //Yoksa exception fırlatıyoruz
                {
                    throw new Exception("Bu isme sahip bir kahraman listede yok");
                }
                _context.Remove(hero); // Bulduysak siliyoruz
                var success = await _context.SaveChangesAsync() > 0; //Unit tipinden bir şey döndürmemiz lazım
                if (success)
                {
                    return Unit.Value;
                }
                throw new Exception("Silme işlemi sırasında bilinmeyen hata.");
            }
        }
    }
}
