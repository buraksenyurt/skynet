using System;
using Xunit;
using MusicBox;

namespace MusicBox.Test
{
    public class MusicianTest
    {
        [Fact]
        public void Elvis_Should_Sing_A_Song()
        {
            ArtistService service=new ArtistService();
            string expected="It's a long day...";
            string response=service.Play("Elvis","It's a long day...");
            Assert.Equal(expected,response);
        }
    }
}
