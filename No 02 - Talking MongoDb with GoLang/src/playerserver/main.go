package main

import (
	"context"
	"fmt"
	"net"
	"os"
	"os/signal"

	playerpb "gRPC-sample/proto"

	"google.golang.org/grpc"
)

/* proto'dan otomatik üretilen player.pb.go içerisindeki RegisterPlayerServiceServer metoduna bir bakın.
Pointer olarak gelen grpc server nesnesi ikinci parametre olarak gelen tipi register etmek için kullanılır.
Bir nevi interface üzerinden enjekte işlemi yaptığımızı düşünebilir miyiz?
*/
type PlayerServiceServer struct{}

func main() {
	fmt.Println("Sunucu 5555 nolu porttan dinleme yapacak...")

	// TCP üzerinden 5555 nolu portu dinleyecek olan nesne oluşturuluyor
	server, err := net.Listen("tcp", ":5555")
	// Olası bir hata durumunu kontrol ediyoruz
	if err != nil {
		fmt.Printf("5555 dinlenemiyor: %v", err)
	}

	// gPRC sunucusu için kayıt(register) işlemleri
	options := []grpc.ServerOption{}
	// yeni bir grpc server oluşturulur
	grpcServer := grpc.NewServer(options...)
	// Bir PlayerService tipi oluşturulur
	playerServiceType := &PlayerServiceServer{}
	// servis sunucu ile birlikte kayıt edilir
	playerpb.RegisterPlayerServiceServer(grpcServer, playerServiceType)

	// CTRL+C ile başlayan kapatma operasyonu
	cnl := make(chan os.Signal)      // işletim sisteminde sinyal alabilmek için bir kanal oluşturduk
	signal.Notify(cnl, os.Interrupt) // CTRL+C mesajı gelene kadar ana rutin açık kalacak
	<-cnl

	fmt.Println("Sunucu kapatılıyor...")
	server.Close()
	fmt.Println("GoodBye Crow")
}

/*
PlayerServiceServer'ın uygulanması gereken metodlarını. Yani servis sözleşmesinin tüm operasyonları
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
