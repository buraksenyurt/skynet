using MediatR;
using System;
using Microsoft.EntityFrameworkCore;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Marvil.Model;

namespace Marvil.Handler
{
    /* 
        Genel pratik olarak ID bazlı arama yaptırıp tek bir kahraman bilgisini geri döndüren davranışı programlıyoruz.
    */
    public class Single
    {
        /*
            CQRS'in Query nesnesi gibi düşünelim.
            Hero tipinden bir nesne döndüreceği belirtiliyor
            İsme göre arama yapacağımız için Name isimli bir özellik de var.
        */
        public class Query : IRequest<Hero>
        {
            public string Name { get; set; }
        }

        public class Handler : IRequestHandler<Query, Hero>
        {
            private MarvilDbContext _context { get; }
            public Handler(MarvilDbContext context)
            {
                _context = context;
            }
            public async Task<Hero> Handle(Query request, CancellationToken cancellationToken)
            {
                var hero = await _context.Heroes.FirstOrDefaultAsync(h => h.Name == request.Name);
                if (hero == null)
                    throw new Exception("Aranan kahraman bulunamadı");
                return hero;
            }
        }
    }
}