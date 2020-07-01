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
	GuessTheNumber()
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
