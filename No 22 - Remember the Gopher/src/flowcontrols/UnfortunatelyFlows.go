package main

import (
	"bufio"
	"math/rand"
	"os"
	"strconv"
	"strings"
	"time"
)

/*
	Pek tabii bir programlama dilinin olmazsa olmazları karşılaştırma operatörleri ve döngüler.
	Bu kod parçasında if, for, switch kullanımları için basit kullanımlar yer alıyor.
*/

func main() {
	//GuessTheNumber()
	HowManyStoryPoint()
}

//HowManyStoryPoint metotunda temel bir switch case kullanımı söz konusudur
func HowManyStoryPoint() {

	println("İzleme ekranında sunucuların işlemci ve ram kullanımlarını grafiksel olarak görmek istiyorum.")
	println("Sence bu hikaye kaç story point eder? (1,3,5,8,13,20,100)")

	// Kullanıcının tahminini istedik
	reader := bufio.NewReader(os.Stdin)
	input, _ := reader.ReadString('\n')
	input = strings.Replace(input, "\n", "", -1)

	// int'e çevrilebilir olması önemli
	point, err := strconv.Atoi(input)
	if err != nil {
		println("Bir sayı girmeliydin :(")
	} else {

		// işte switch bloğu
		switch point {
		case 1, 3, 5: // 1,3,5 değerleri için aynı case geçerli
			println("Oldukça makul")
			break
		case 8:
			println("Biraz zorlayacak gibi")
			break
		case 13, 20:
			println("Konuyu biraz daha detaylandırmak lazım.")
			break
		case 100:
			println("Oooo...Epik bir konu bu. Parçalamak gerekiyor")
		default: // Üstteki case'lerin tamamı atlanırsa varsayılan blok çalışacak
			println("Büyük ihtimalle izin verilen sayılardan birisini girmedin")
			break
		}
	}

}

//GuessTheNumber sayı tahmin oyunu için kullanılan basit bir fonksiyondur
func GuessTheNumber() {

	println("Şimdi aklımdan 1 ile 10 arasında bir sayı tutacağım. 3 deneme hakkın var.")
	// 3ten geriye doğru sayan bir döngü yaptık.
	// 3ten geriye doğru sayacak.
	for i := 3; i > 0; i-- {
		time.Sleep(1 * time.Second) //Thread üstünde 1 saniyelik duraksatma
		println(i, "...")
	}

	// Gerçekten her seferinde rastgele bir sayı ürettirmek için Seed metodundan yararlanıyoruz.
	rand.Seed(time.Now().UnixNano())
	number := rand.Intn(10)
	println("Tuttum. Hadi bilmeye çalış")

	reader := bufio.NewReader(os.Stdin) // kullanıcının terminal girişini okumak için kullanacağımız reader nesnesi

	tryCount := 3
	// Sonsuz döngümüz
	for {

		// Deneme sayısını baştan kontrol ediyorum
		if tryCount == 0 {
			println("Üzgünüm :( 3 hakkını da kullandın :(")
			break // Tahmin hakkımız dolduysa döngüden çıkıyoruz
		}

		println("Lütfen tahminini gir...", tryCount, " hakkın var.")
		input, _ := reader.ReadString('\n')
		input = strings.Replace(input, "\n", "", -1)

		// Önce ekrandan girilen değeri strconv paketindeki Atoi fonksiyonu ile int tipine dönüştürüyoruz.
		// Dönüştürme işlemi başarılı olmazsa err değeri nil olmayacaktır. Bu durumda metoddan çıkıyoruz.
		guess, err := strconv.Atoi(input)
		if err != nil {
			println("Bir sayı girmeliydin :(")
			break
		}

		// Girilen sayı tahmin edilen sayı mı?
		if number == guess {
			println("BRAVO SANA! Bildin ;)")
			break //Döngüden çıkarma
		} else { //else'in } { arasında yazılması gerekiyor. Aksi durum syntax hatası
			tryCount-- //Bilemeyince deneme sayısını 1 azalt
		}
	}
}
