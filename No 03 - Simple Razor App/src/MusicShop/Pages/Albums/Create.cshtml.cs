using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.AspNetCore.Mvc.Rendering;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Logging;
using MusicShop.Data;
using MusicShop.Models;
using System.IO;

namespace MusicShop.Pages.Albums
{
    public class CreateModel : PageModel
    {
        private readonly ILogger<CreateModel> _logger;
        private readonly MusicShopContext _context;
        [BindProperty]
        public Album AlbumInstance { get; set; }
        [BindProperty]
        public IFormFile CoverPhotoFile { get; set; }
        public SelectList ArtistList { get; set; }

        public CreateModel(ILogger<CreateModel> logger, MusicShopContext context)
        {
            _logger = logger;
            _context = context;
        }

        public void OnGet()
        {
            var artistNamesQuery = from a in _context.Artist
                                   orderby a.Name
                                   select a;

            ArtistList = new SelectList(artistNamesQuery, "ID", "Name");
        }

        public async Task<IActionResult> OnPostAsync()
        {
            var newAlbum = new Album();

            _logger.Log(LogLevel.Warning, $"Uploaded file length {CoverPhotoFile.Length.ToString()} bytes");

            if (await TryUpdateModelAsync<Album>(
                 newAlbum,
                 "albuminstance",
                 a => a.Title, a => a.ArtistID, a => a.ReleaseYear))
            {
                using (var memoryStream = new MemoryStream())
                {
                    await CoverPhotoFile.CopyToAsync(memoryStream);
                    newAlbum.Cover = memoryStream.ToArray();
                }
                _context.Album.Add(newAlbum);
                await _context.SaveChangesAsync();
            }

            return RedirectToPage("./Index");
        }
    }
}
