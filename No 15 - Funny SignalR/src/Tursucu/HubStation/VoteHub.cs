using Microsoft.AspNetCore.SignalR;
using System.Threading.Tasks;

namespace Tursucu.HubStation
{
    public class VoteHub
        : Hub // SignalR başkanı olmanın doğası Hub sınıfından türemektir.
        // Hub sınıfı mesajlaşma alt yapısını ve mesaj dağıtımını kolaylaştırır
    {
        // İstemci ile sunucunun eş zamanlı konuşmasının doğası gereği
        // Asenkron bir metodumuz var.
        // Metot adı istemci tarafındaki Javascript için önemli (invoke kısmına bak)
        public async Task PushVoteMessage(string user,string userChoice)
        {
            // user : Kimden mesaj geliyor
            // userChoice : kullanıcı hangi seçeneği seçiyor. Sirke mi limon mu?
            // GetVoteMessage ismi önemli. Javascript tarafındaki on event'inde yakalancak
            // All ile bağlı olan tüm kullanıcıları gösterdik
            // ve SendAsync ile hepsine GetVote isimli bir mesaj yayınladık
            // Şayet karşı tarafta bağlanıp da bu olayı dinleyen varsa yaşadı
            await Clients.All.SendAsync("GetVoteMessage",user,userChoice);   
        }
    }
}