package main

import (
	"context"
	"fmt"
	"net"
	"os"
	"os/signal"
	"strings"

	"go.mongodb.org/mongo-driver/bson/primitive"

	playerpb "gRPC-sample/proto"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
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

/* Protobuf mesajlarında taşınan serileşmiş içeriği nesnel olarak ele alacağımı struct */
type Player struct {
	ID       primitive.ObjectID `bson:"_id,omitempty"` // MongoDB tarafındaki ObjectId değerini taşır
	PlayerID string             `bson:"player_id"`
	Fullname string             `bson:"fullname"`
	Position string             `bson:"position"`
	Bio      string             `bson:"bio"`
}

/* PlayerServiceServer'ın uygulanması gereken metodlarını. Yani servis sözleşmesinin tüm operasyonları
 */

// Yeni bir oyuncu eklemek için kullanacağımız fonksiyon
func (srv *PlayerServiceServer) AddPlayer(ctx context.Context, req *playerpb.AddPlayerReq) (*playerpb.AddPlayerRes, error) {
	payload := req.GetPlyr() // GetPlyr (GetPlayer değil o servis metodumuz) fonksiyonu ile request üzerinden gelen player içeriği çekilir

	// İçerik ile gelen alan değerleri player struct nesnesini oluşturmak için kullanılır
	player := Player{
		PlayerID: payload.GetPlayerId(),
		Fullname: payload.GetFullname(),
		Position: payload.GetPosition(),
		Bio:      payload.GetBio(),
	}

	// player nesnesi mongodb veritabanındaki koleksiyona kayıt edilir
	result, err := playerCollection.InsertOne(mongoContext, player)

	// bir problem oluştuysa
	if err != nil {
		// gRPC hatası döndürülür
		return nil, status.Errorf(
			codes.Internal,
			fmt.Sprintf("Bir hata oluştu : %v", err),
		)
	}

	// Hata oluşmadıysa koleksiyona eklenen yeni doküman
	// üretilen ObjectID değeri de atanarak geri döndürülür
	objectID := result.InsertedID.(primitive.ObjectID)
	payload.Id = objectID.Hex()
	return &playerpb.AddPlayerRes{Plyr: payload}, nil
}

func (srv *PlayerServiceServer) EditPlayer(ctx context.Context, req *playerpb.EditPlayerReq) (*playerpb.EditPlayerRes, error) {
	return nil, nil
}

func (srv *PlayerServiceServer) RemovePlayer(ctx context.Context, req *playerpb.RemovePlayerReq) (*playerpb.RemovePlayerRes, error) {
	// önce silinmek istenen playerId bilgisi alınır
	id := strings.Trim(req.GetPlayerId(), "\t \n")
	fmt.Println(id)
	// DeleteOne metodu ile silme operasyonu gerçekleştirilir
	_, err := playerCollection.DeleteOne(ctx, bson.M{"player_id": id})

	// hata kontrolü yapılıyor
	if err != nil {
		return nil, status.Errorf(codes.NotFound, fmt.Sprintf("Silinmek istenen oyuncu bulunamadı. %s", err))
	}

	// hata yoksa işlemin başarılı olduğuna dair sonuç dönülür
	return &playerpb.RemovePlayerRes{
		Removed: true,
	}, nil
}

// MongoDB'deki ID bazlı olarak oyuncu verisi döndüren metodumuz
func (srv *PlayerServiceServer) GetPlayer(ctx context.Context, req *playerpb.GetPlayerReq) (*playerpb.GetPlayerRes, error) {
	// request ile gelen player_id bilgisini alıyoruz
	// Trim işlemi önemli. İstemci terminalden değer girdiğinde alt satıra geçme işlemi söz konusu.
	// Veri bu şekilde gelirse kayıt bulunamaz. Dolayısıyla bir Trim işlemi yapıyoruz
	id := strings.Trim(req.GetPlayerId(), "\t \n")
	// bson.M metoduna ilgili sorguyu ekleyerek oyuncuyu koleksiyonda arıyoruz
	result := playerCollection.FindOne(ctx, bson.M{"player_id": id})

	player := Player{}
	// bulunan oyuncu decode metodu ile ters serileştirilip player değişkenine alınır
	if err := result.Decode(&player); err != nil {
		return nil, status.Errorf(codes.InvalidArgument, fmt.Sprintf("Sanırım aranan oyuncu bulunamadı %v", err))
	}

	// Decode işlemi başarılı olur ve koleksiyondan bulunan içerik player isimli değişkene ters serileşebilirse
	// artık dönecek response nesne içeriğini hazırlayabiliriz
	res := &playerpb.GetPlayerRes{
		Plyr: &playerpb.Player{
			Id:       player.ID.Hex(),
			PlayerId: player.PlayerID,
			Fullname: player.Fullname,
			Position: player.Position,
			Bio:      player.Bio,
		},
	}
	return res, nil
}

// Tüm oyuncu listesini stream olarak dönen metod
func (srv *PlayerServiceServer) GetPlayerList(req *playerpb.GetPlayerListReq, stream playerpb.PlayerService_GetPlayerListServer) error {

	currentPlayer := &Player{}

	// Find metodu veri üzerinden hareket edebileceğimiz bir Cursor nesnesi döndürür
	// bu cursor nesnesi sayesinde istemciye tüm oyuncu listesini bir seferde göndermek yerine
	// birer birer gönderme şansına sahip olacağız
	// Bu nedenle sunucu bazlı bir streamin stratejimiz var
	cursor, err := playerCollection.Find(context.Background(), bson.M{})
	if err != nil {
		return status.Errorf(codes.Internal, fmt.Sprint("Bilinmeyen hata oluştu"))
	}

	// metod işleyişini tamamladığında cursor nesnesini kapatacak çağrıyı tanımlıyoruz
	defer cursor.Close(context.Background())

	// iterasyona başlanır ve Next true döndüğü sürece devam eder
	// yani okunacak mongodb dokümana kalmayana dek
	for cursor.Next(context.Background()) {
		// cursor verisini currentPlayer nesnesine açıyoruz
		cursor.Decode(currentPlayer)
		// istemciye mongodb'den gelen güncel oyuncu bilgisinden yararlanarak cevap dönüyoruz
		stream.Send(&playerpb.GetPlayerListRes{
			Plyr: &playerpb.Player{
				Id:       currentPlayer.ID.Hex(),
				PlayerId: currentPlayer.PlayerID,
				Fullname: currentPlayer.Fullname,
				Position: currentPlayer.Position,
				Bio:      currentPlayer.Bio,
			},
		})
	}

	return nil
}
