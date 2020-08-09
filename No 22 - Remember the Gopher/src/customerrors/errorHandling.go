package main

import (
	"errors"
	"fmt"
)

/*
	Go dilinde exception'lar yoktur.
	Diğer yandan birçok built-in fonksiyon dönüşünde error nesnesi kontrol edilerek o fonksiyon işleyişinde hata olup olmadığı kontrol edilir.
	İzleyen örneklerde error fırlatmak ile ilgili işlemler yer alıyor.
*/

func main() {

	/*
		Örneğin bu ilk örnekte Player nesnesi örnekleyen createPlayer metodunun döndürdüğü err nesnesi kontrol edilmekte.
		Eğer nickName bilgisi olmadan bir oyuncu oluşturulmaya çalışılırsa createPlayer metodu içinden errors.New ile bir hata mesajı fırlatılıyor.
		Eğer bir sorun yoksa oyuncu nesnesi oluşturuluyor.
	*/
	newPlayer, err := createPlayer("", 100)
	// newPlayer, err := createPlayer("Mine Crafter", 100) // Kodun sağlaması için açılabilecek örnek
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Printf("Oyuncu bilgileri: %s %v \n", newPlayer.nickName, newPlayer.level)
	}

	/*
		Aşağıdaki kod parçasında ise kendi error nesnemizi kullanıyoruz. (.Netçiler için bir nevi kendi Exception sınıfını yazmak gibi)
	*/
	s, err := createSquare(-5, 4)
	// s, err := createSquare(5, 4) // Sağlaması için kullanın
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Printf("%v %v boyutlarında dörtgen oluştu\n", s.x, s.y)
	}
}

//Player tipi
type Player struct {
	nickName string
	level    int
}

//createPlayer metodu dikkat edileceği üzere iki değer döndürmekte
//İlki oyuncu ama ikincisi bir aksilik olması halinde hata bilgisini taşıyan error nesnesi
func createPlayer(nick string, lvl int) (p Player, err error) {

	//Eğer nick değişkeni boş geldiyse
	if nick == "" {
		// Yeni bir hata oluşturuluyor.
		err = errors.New("Bir nickname verilmeli")
	} else {
		p = Player{nick, lvl}
	}

	return p, err
}

/*
	Kendi Error nesnemizi bir struct olarak tanımlayı, Error fonksiyonelliği kazandırıyoruz
*/

type InvalidSizeError struct{}

//Error metodunu InvalidSizeError isimli struct için uyguladık.
//Tahmin edeceğiniz üzere Error metodu built-in error interface içinde tanımlıdır.
func (e InvalidSizeError) Error() string {
	return "Boyutlar geçersiz"
}

type Square struct {
	x, y int
}

func createSquare(a, b int) (s Square, err error) {
	/*
		fonksiyon bir Square değişkeni oluşturmak için kullanılıyor.
		Ne var ki gelen a ve b değerlerinden herhangi biri 0'a eşit veya küçükse
		InvalidSizeError nesnesini dolduruyoruz. Böylece işlemde hata olduğunu belirtmiş oluyoruz.
	*/
	if a <= 0 || b <= 0 {
		err = InvalidSizeError{}
	} else {
		s = Square{a, b}
	}
	return s, err
}
