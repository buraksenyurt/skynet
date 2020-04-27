using DemographicAPI.Model;
using Microsoft.AspNetCore.Mvc;

namespace DemographicAPI.Controller
{
    [Route("api/[controller]")]
    [ApiController]
    public class PhoneCodesController : ControllerBase
    {
        [HttpGet("{code:length(3)}")]
        public ActionResult<CountryCode> Get(string code)
        {
            var countryCode = new CountryCode
            {
                Abbrevation = "TR",
                Name = "Türkiye",
                PhoneCode = "90"
            };
            return countryCode;
        }
    }
}