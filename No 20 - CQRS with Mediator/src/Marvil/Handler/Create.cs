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
    Yeni veri ekleme bizim için Create isimli bir davranış
    */
    public class Create
    {
        /*
            Command sınıfı MediatR'deki IRequest arayüzünden türer.
            CQRS'in Command nesneleri bilindiği üzere geriye bir şey döndürmeyen aksiyonlarda ele alınır. Veri ekleme gibi.
            Bu nedenle List ve Single sınıflarındaki Query tiplerinde olduğu gibi türlü bir IRequest söz konusu değildir.
            Hero sınıfı ile aynı özelliklere sahiptir.
            Handler ile ilişkilendiriecek Command nesnesidir.
        */
        public class Command : IRequest
        {
            public string Name { get; set; }
            public string AlterEgo { get; set; }
            public int ForceLevel { get; set; }
        }

        /*
            Handler sınıfı IRequestHandler<Command> arayüzünü uygulamakta
        */
        public class Handler : IRequestHandler<Command>
        {
            // DbContext'in enjekte edilmesi
            private readonly MarvilDbContext _context;
            public Handler(MarvilDbContext context)
            {
                _context = context;
            }

            /*
                Yeni kahraman ekleme işini ele alan metodumuz.
                İlk parametre aynı zamanda gelen talepteki bilgileri alıp yeni Hero nesnesinin örneklenmesin kullanılıyor
            */
            public async Task<Unit> Handle(Command request, CancellationToken cancellationToken)
            {
                //TODO: Kahraman daha önceden eklenmişse tekrar eklenmesin

                var hero = new Hero
                {
                    Name = request.Name,
                    AlterEgo = request.AlterEgo,
                    ForceLevel = request.ForceLevel
                };
                // DbContext üstündeki Heroes koleksiyonuna ekleniyor
                _context.Heroes.Add(hero);
                // Kayıt işlemi başarılıysa 
                var success = await _context.SaveChangesAsync() > 0;
                if (success)
                {
                    return Unit.Value;
                }
                else // Değilse
                {
                    throw new Exception("Kahraman listeye eklenemedi");
                }
            }
        }
    }
}