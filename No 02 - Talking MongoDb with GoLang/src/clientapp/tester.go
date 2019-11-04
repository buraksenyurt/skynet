package main

import (
	"bufio"
	"context"
	"fmt"
	"os"
	"strings"

	playerpb "gRPC-sample/proto"

	"google.golang.org/grpc"
)

func main() {
	var client playerpb.PlayerServiceClient
	var reqOptions grpc.DialOption

	// HTTPS ayarları ile uğraşmak istemedim
	reqOptions = grpc.WithInsecure()
	// gRPC servisi ile el sıkışmaya çalışıyoruz
	// Hatırlanacağı üzere sunucu 5555 nolu porttan hizmet veriyordu
	connection, err := grpc.Dial("localhost:5555", reqOptions)
	if err != nil {
		fmt.Println(err)
		return
	}
	// proxy nesnesini ilgili bağlantıyı kullanacak şekilde örnekliyoruz
	client = playerpb.NewPlayerServiceClient(connection)

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
	} else {
		// Eğer bir hata oluşmadıysa MongoDB tarafından üretilen ID değerini ekranda görmemiz lazım
		fmt.Printf("%s ile yeni oyuncu eklendi \n", res.Plyr.Id)
	}
}
