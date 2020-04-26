using Alfabe.Core;
using Microsoft.AspNetCore.Mvc;
using System.Collections.Generic;

namespace Alfabe.API.Controllers
{
    [Route("nba/api/[controller]")]
    [ApiController]
    public class TeamsController 
        : ControllerBase
    {
        readonly TeamService _service;

        public TeamsController(IMongoDbSettings settings)
        {
            _service = new TeamService(settings);
        }

        [HttpGet]
        public ActionResult<List<Team>> Get()=> _service.GetAll();

        [HttpGet("{id:length(24)}")]
        public ActionResult<Team> Get(string id) => _service.GetSingle(id);

        [HttpPost]
        public ActionResult<Team> Create(Team team) => _service.Create(team);

        [HttpPut("{id:length(24)}")]
        public IActionResult Update(string id,Team currentTeam)
        {
            var team = _service.GetSingle(id);
            if (team == null)
                return NotFound();

            _service.Update(id, currentTeam);
            return Ok();
        }

        [HttpDelete("{id:length(24)}")]
        public IActionResult Delete(string id)
        {
            var team = _service.GetSingle(id);
            if (team == null)
                return NotFound();

            _service.Delete(id);
            return Ok();
        }
    }
}