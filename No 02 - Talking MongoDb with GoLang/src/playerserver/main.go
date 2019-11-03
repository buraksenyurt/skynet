package main

import (
	"context"
	"fmt"
	"net"
	"os"
	"os/signal"

	playerpb "gRPC-sample/proto"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"google.golang.org/grpc"
)

/* proto'dan otomatik üretilen player.pb.go içerisindeki RegisterPlayerServiceServer metoduna bir bakın.
Pointer olarak gelen grpc server nesnesi ikinci parametre olarak gelen tipi register etmek için kullanılır.
Bir nevi interface üzerinden enjekte işlemi yaptığımızı düşünebilir miyiz?
*/
type PlayerServiceServer struct{}

var db *mongo.Client
var playerCollection *mongo.Collection
var mongoContext context.Context

func main() {
	// TCP üzerinden 5555 nolu portu dinleyecek olan nesne oluşturuluyor
	server, err := net.Listen("tcp", ":5555")
	// Olası bir hata durumunu kontrol ediyoruz
	if err != nil {
		fmt.Printf("5555 dinlenemiyor: %v", err)
	}

	// gPRC sunucusu için kayıt(register) işlemleri
	grpcOptions := []grpc.ServerOption{}
	// yeni bir grpc server oluşturulur
	grpcServer := grpc.NewServer(grpcOptions...)
	// Bir PlayerService tipi oluşturulur
	playerServiceType := &PlayerServiceServer{}
	// servis sunucu ile birlikte kayıt edilir
	playerpb.RegisterPlayerServiceServer(grpcServer, playerServiceType)

	// mongoDB bağlantı işlemleri
	fmt.Println("MongoDB sunucusuna bağlanılıyor")
	mongoContext = context.Background()
	// bağlantı deneniyor
	db, err = mongo.Connect(mongoContext, options.Client().ApplyURI("mongodb://localhost:27017"))
	// olası bir bağlantı hatası varsa
	if err != nil {
		fmt.Println(err)
	}
	// Klasik ping metodunu çağırıyoruz
	err = db.Ping(mongoContext, nil)
	if err != nil {
		fmt.Println(err)
	} else {
		// çağrı başarılı olursa bağlandık demektir
		fmt.Println("MongoDB ile bağlantı sağlandı")
	}
	// nba isimli veritabanındaki player koleksiyonuna ait bir nesne örnekliyoruz
	// veritabanı ve koleksiyon yoksa oluşturulacaktır
	playerCollection = db.Database("nba").Collection("player")

	// gRPC sunucusunu aktif olan TCP sunucusu içerisinde bir child routine olarak başlatıyoruz
	go func() {
		if err := grpcServer.Serve(server); err != nil {
			fmt.Println(err)
		}
	}()
	fmt.Println("Sunucu 5555 nolu porttan gPRC tabanlı iletişime hazır.\nDurdurmak için CTRL+C.")

	// CTRL+C ile başlayan kapatma operasyonu
	cnl := make(chan os.Signal)      // işletim sisteminde sinyal alabilmek için bir kanal oluşturduk
	signal.Notify(cnl, os.Interrupt) // CTRL+C mesajı gelene kadar ana rutin açık kalacak
	<-cnl

	fmt.Println("Sunucu kapatılıyor...")
	grpcServer.Stop() // gRPC sunucusunu durdur
	server.Close()    // TCP dinleyicisini kapat
	fmt.Println("GoodBye Crow")
}

/* PlayerServiceServer'ın uygulanması gereken metodlarını. Yani servis sözleşmesinin tüm operasyonları
 */
func (srv *PlayerServiceServer) AddPlayer(ctx context.Context, req *playerpb.AddPlayerReq) (*playerpb.AddPlayerRes, error) {
	return nil, nil
}

func (srv *PlayerServiceServer) EditPlayer(ctx context.Context, req *playerpb.EditPlayerReq) (*playerpb.EditPlayerRes, error) {
	return nil, nil
}

func (srv *PlayerServiceServer) RemovePlayer(ctx context.Context, req *playerpb.RemovePlayerReq) (*playerpb.RemovePlayerRes, error) {
	return nil, nil
}

func (srv *PlayerServiceServer) GetPlayer(ctx context.Context, req *playerpb.GetPlayerReq) (*playerpb.GetPlayerRes, error) {
	return nil, nil
}

func (srv *PlayerServiceServer) GetPlayerList(ctx context.Context, req *playerpb.GetPlayerListReq) (*playerpb.GetPlayerListRes, error) {
	return nil, nil
}
