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
        Güncelleme tasarlandığı kısım.
    */
    public class Update
    {
        /*
            Kahraman verisi güncellenirken kuvvetle muhtemel tüm özelliklerinin son hallerini almak lazım.
            Güncelleme CQRS'in Command kısmına ait bir konu olduğundan geriye bir şey döndürmeyeceğiz.
            Bu nedenle sadece IRequest türetmesi söz konusu.
        */
        public class Command : IRequest
        {
            public int Id { get; set; }
            public string Name { get; set; }
            public string AlterEgo{get;set;}
            public int ForceLevel { get; set; }
        }

        // Güncelleme işini üstlenen Handler
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
                var hero = await _context.Heroes.FirstOrDefaultAsync(h => h.Id == request.Id);
                if (hero == null) //Yoksa exception fırlatıyoruz
                {
                    throw new Exception("Bu isme sahip bir kahraman listede yok");
                }
                // varsa güncelleme yapıp kaydediyoruz.
                hero.Name=request.Name;
                hero.AlterEgo=request.AlterEgo;
                hero.ForceLevel=request.ForceLevel;

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
