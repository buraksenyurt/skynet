package main

import (
	"bufio"
	"context"
	"fmt"
	"io"
	"os"
	"strings"

	playerpb "gRPC-sample/proto"

	"google.golang.org/grpc"
)

var client playerpb.PlayerServiceClient
var reqOptions grpc.DialOption

func main() {

	// HTTPS ayarları ile uğraşmak istemedim
	reqOptions = grpc.WithInsecure()
	// gRPC servisi ile el sıkışmaya çalışıyoruz
	connection, err := grpc.Dial("localhost:5555", reqOptions)
	if err != nil {
		fmt.Println(err)
		return
	}
	// proxy nesnesini ilgili bağlantıyı kullanacak şekilde örnekliyoruz
	client = playerpb.NewPlayerServiceClient(connection)

	// Oyuncu ekleyelim
	insertPlayer()
	// tüm oyuncu listesini çekelim
	getAllPlayerList()

	// sembolik olarak ID bazlı 3 oyuncu aratalım
	for i := 0; i < 3; i++ {
		reader := bufio.NewReader(os.Stdin)
		fmt.Println("Oyuncu IDsini gir.")
		playerID, _ := reader.ReadString('\n')
		getByPlayerID(playerID)
	}

	// Silme operasyonunu deniyoruz
	reader := bufio.NewReader(os.Stdin)
	fmt.Println("Silmek istediğiniz oyuncunun IDsini girin.")
	playerID, _ := reader.ReadString('\n')
	removePlayerByID(playerID)
}

func insertPlayer() {
	// Yeni oyuncu eklenmesi için deneme kodu
	// Veri ihlalleri örneğin basitliği açısından göz ardı edilmiştir
	reader := bufio.NewReader(os.Stdin)
	fmt.Println("Yeni oyuncu girişi")
	fmt.Println("Id->")
	id, _ := reader.ReadString('\n')
	id = strings.Replace(id, "\n", "", -1)
	fmt.Println("Adı->")
	fullname, _ := reader.ReadString('\n')
	fullname = strings.Replace(fullname, "\n", "", -1)
	fmt.Println("Pozisyon->")
	position, _ := reader.ReadString('\n')
	position = strings.Replace(position, "\n", "", -1)
	fmt.Println("Kısa biografisi->")
	bio, _ := reader.ReadString('\n')
	bio = strings.Replace(bio, "\n", "", -1)

	// protobuf dosyasındaki şemayı kullanarak örnek bir oyuncu nesnesi örnekliyoruz
	newPlayer := &playerpb.Player{
		PlayerId: id,
		Fullname: fullname,
		Position: position,
		Bio:      bio,
	}

	// servisin AddPlayer metodunu o anki context üzerinden çalıştırıp
	// request payload içerisinde yeni oluşturduğumuz nesneyi gönderiyoruz
	res, err := client.AddPlayer(
		context.TODO(),
		&playerpb.AddPlayerReq{
			Plyr: newPlayer,
		},
	)
	if err != nil {
		fmt.Println(err)
		return
	}
	// Eğer bir hata oluşmamışsa MongoDB tarafından üretilen ID değerini ekranda görmemiz lazım
	fmt.Printf("%s ile yeni oyuncu eklendi \n", res.Plyr.Id)
}

// Tüm oyuncu listesini çektiğimiz metod
func getAllPlayerList() {

	// önce request oluşturulur
	req := &playerpb.GetPlayerListReq{}

	// proxy nesnesi üzerinden servis metodu çağrılır
	s, err := client.GetPlayerList(context.Background(), req)
	if err != nil {
		fmt.Println(err)
		return
	}

	// sunucu tarafından stream bazlı dönüş söz konusu
	// yani kaç tane oyuncu varsa herbirisi için sunucudan istemciye
	// cevap dönecek
	for {
		res, err := s.Recv() // Recv metodu player.pb.go içerisine otomatik üretilmiştir. İnceleyin ;)
		if err != io.EOF {   // döngü sonlanmadığı sürece gelen cevaptaki oyuncu bilgisini ekrana yazdırır
			fmt.Printf("[%s] %s - %s \n\n", res.Plyr.PlayerId, res.Plyr.Fullname, res.Plyr.Bio)
		} else {
			break
		}
	}
}

// Oyuncuyu PlayerID değerinden bulan metodumuz
func getByPlayerID(playerID string) {
	// parametre olarak gelen playerID değerinden bir request oluşturulur
	req := &playerpb.GetPlayerReq{
		PlayerId: playerID,
	}
	// GetPlayer servis metoduna talep gönderilir
	res, err := client.GetPlayer(context.Background(), req)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println(res.Plyr.Fullname)
}

// Oyuncu silme fonksiyonumuz
func removePlayerByID(playerID string) {
	// RemovePlayer servis çağrısı için gerekli Request tipi hazırlanır
	req := &playerpb.RemovePlayerReq{
		PlayerId: playerID,
	}
	// servisi çağrısı yapılıp sonucu kontrol edilir
	_, err := client.RemovePlayer(context.Background(), req)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println("Oyuncu silindi")
}
