package main

import (
	"fmt"
	"time"
)

/*
	Go dilinin belki de en başarılı olduğu konuların başında concurrency(eş zamanlı çalıştırma) yer alıyor.
	Channel ve GoRoutine nesnelerinin kullanımı burada önemli. GoRoutine'ler hafif siklet thread'ler olarak düşünülebilir. Maliyetleri ucuz.
	Channel nesneleri, başlatılan GoRoutine'lerle veri alışverişini tesis eder ve senkronizasyonu sağlar.
	Aşağıdaki örnekte eş zamanlı başlatılan GoRoutine'ler var.
*/

func main() {
	start()
}

func start() {
	/*
		Şimdi aşağıdaki GoRoutine'leri ele alalım.
		Sayı arttıran bir fonkisyonu ve sonrasında isimsiz fonksiyon gövdesine sahip bir bloğu GoRoutine olarak başlatıyoruz.

		Kodu çalıştırdıktan sonra oluşan ekran görüntüsüne de dikkat edelim.
		İşlemcinin durumuna göre metot sıraları değişmiştir. Bu farklılık, Concurrent çalışmanın minik bir ispatıdır da aynı zamanda.

	*/
	panama := make(chan int)       //int değerler taşıyan bir kanal nesnesi
	go increaseNumber(100, panama) //go ile bir GoRoutine başlatıyoruz
	go increaseNumber(200, panama) //ikinci bir GoRoutine daha başlatılıyor
	go increaseNumber(300, panama) //Üçüncüsü de başlatılıyor
	go increaseNumber(400, panama) //dördüncüsü de başlatılıyor

	//GoRoutin'leri aşağıdaki gibi isimsiz fonksiyon olarak da başlatabiliriz
	capeHorn := make(chan string)
	go func() { capeHorn <- "Bir haber başlığı" }() //fonksiyon içerisinden capeHorn isimli kanala bir bilgi yazdırılıyor

	fmt.Println(<-panama, <-panama, <-panama, <-panama) // dört GoRoutine başlatmıştık. Herbiri için kanala bırakılan verileri okurken <- operatörünü nasıl kullandığımıza dikkat edelim.
	fmt.Println(<-capeHorn)                             //ve kanala yazılan bilgiyi de burada alıyoruz

	/*
		Bir diğer kullanım şekli. Broadcasting benzeri.
		SendNewsHeader ile 10 saniye sonra ekrana basılacak bir haber başlığı söz konusu.
	*/
	editor := SendNewsHeader("Yazıyor...Yazıyor...Jules Verne'nin son kitabı çıktı. Yazıyor...", 7*time.Second)

	// Kod buradan akmaya devam edecektir. SendNewsHeader'daki 7 saniyelik duraksama burayı engellemez. Ayrı bir GoRoutine çünkü.
	fmt.Println("Bakalım buralarda 10 saniye içinde bir şeyler olacak mı?")

	/*
		Yukarıda başlatılan GoRoutine sonucu beklenirken biz eş zamanlı başka GoRoutine'ler başlatıp,
		bitenlerden birisinin sonucunu select...case ile nasıl alabileceğimize bakalım.
		Select sadece channel'lar için kullanılmaktadır.
	*/

	// Üç kanal oluşturduk
	chn1 := make(chan string)
	chn2 := make(chan int)
	chn3 := make(chan int)

	// Oluşturulan 3 kanaldan ikisini kullanan iki GoRoutine başlattık
	go func() { chn1 <- "Channel 1 içerisine yazılan sonuç" }()
	go func() { chn2 <- 11 }()

	// Hangisi bitmiş bir bakalım
	select {
	case msg := <-chn1: // chn1'e aktarılan değer alınıyor
		fmt.Println(msg)
	case <-chn2: // İlle de biten GoRoutine sonucunu almak zorunda değiliz
		fmt.Println("Chn2 işleyişini bitirdi")
	case <-chn3:
		fmt.Println("chn3 aslında hiç kullanılmadı")
	}

	<-editor // Pek tabii SendNewsHeader içerisindeki işleyiş henüz bitmediyse ana thread'i bu şekilde bekletebiliriz.
}

/*
increaseNumber fonksiyonu normal bir int parametre dışında,
int türünden değerleri taşıyan bir de channel nesnesi alıyor.
Go Routine'ler bu metodu çağırırken bir channel nesnesi üzerinden veri alışverişinde bulunacaklar.
*/
func increaseNumber(n int, chn chan int) {
	/*
		gelen n sayısının değerini bir artırdık ve chn isimli channel'e yazdık. <- send operatörü.
		Sağ taraftaki değeri kanalın içerisine bırakmış olduk.
		Bu fonksiyonu çağıran GoRoutin kimse chn kanal nesnesi üstünden kendisi için bırakılan sayıyı alabilir.
	*/
	chn <- n + 1
}

/*
SendNewsHeader fonksiyonu içinde bir GoRoutine başlatıyor ve geriye struct türünden bir channel döndürüyoruz.

*/
func SendNewsHeader(header string, delay time.Duration) (editor <-chan struct{}) {
	bbc := make(chan struct{})
	//GoRoutine'in başlatıldığı yer
	go func() {
		time.Sleep(delay) // parametre olarak gelen süre kadar bekletme işlemi
		fmt.Println(header)
		close(bbc) // kanalı kapatıyoruz
	}()
	return bbc //fonksiyondan geriye döndürdüğümüz kanal nesnesi
}
