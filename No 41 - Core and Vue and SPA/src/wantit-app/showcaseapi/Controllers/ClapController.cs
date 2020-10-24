using System.Collections.Generic;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using showcaseapi.Data;
using showcaseapi.Model;

namespace showcaseapi.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class ClapsController : ControllerBase
    {
        private readonly WantItDbContext _dbContext;

        public ClapsController(WantItDbContext dbContext)
        {
            _dbContext = dbContext;
        }

        // GET api/claps
        [HttpGet]
        public async Task<ActionResult<List<Clap>>> Get()
        {
            return await _dbContext.Claps.ToListAsync();
        }

        // GET api/claps/{username}
        [HttpGet("{username}")]
        public async Task<ActionResult<Clap>> Get(string username)
        {
            return await _dbContext.Claps.FindAsync(username);
        }

        // POST api/claps
        [HttpPost]
        public async Task Post(Clap payload)
        {
            await _dbContext.AddAsync(payload);
            await _dbContext.SaveChangesAsync();
        }
    }
}