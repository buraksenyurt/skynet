package main

/*
	Pointer'lar GO dilinin atalarından gelen önemli detaylardan birisidir.
	Kısaca bellek adreslerini tutabildiğimiz veri türü olarak düşünülebiliriz.
*/

func main() {
	zeroOne := Player{"Zero One", 10} // Player tipinden değişken tanımladık
	// zeroOne nesnesini new operatörü ile de tanımlayabiliriz. Bu durumda new operatörü değişkenin bellek adresini döndürecektir.
	zeroOne.Nickname = "Zero One"
	zeroOne.Point = 10

	// Etkiyi izleyebilmek için & operatörü ile zeroOne değişkeninin bellek adresini de kullanıyoruz
	println("main içindeki. ", &zeroOne, " ", zeroOne.Nickname, " ", zeroOne.Point)
	increasePoint(zeroOne) // Fonksiyona parametre olarak zeroOne değişkeninin geçirilmesi (birebir kopyalama söz konusu)
	println("main içindeki. ", &zeroOne, " ", zeroOne.Nickname, " ", zeroOne.Point)

	println()
	decreasePoint(&zeroOne) //decrasePoint fonksiyonu Player değişkeni için pointer değeri (bellek adresi) almakta. Bu nedenle & ile zeroOne'un adresini yolladık
	println("main içindeki. ", &zeroOne, " ", zeroOne.Nickname, " ", zeroOne.Point)

	println()
	/*
		new operatörü.
		new operatörü ile bir nesne örneklenirken geriye bellek adresi döndürülür.
	*/
	morris := new(Player)
	morris.Nickname = "morris"
	morris.Point = 18
	println("morris'in adresi(new kullanımı) ", morris) // morris ile doğrudan bellek adresine ulaşmış oluyoruz

	println()
	luckyNumber := 7     //integer bir değişken
	addr := &luckyNumber // değişkenin bellek adresini tutan pointer
	println("luckyNumber'ın(int) bellek adresi ", addr, " İçerisinde duran bilgi ", *addr)
	*addr = 9 // luckyNumber değişkeninin adresini tutan pointer vasıtasıyla değerini de değiştirebiliriz
	println("luckyNumber'ın yeni değeri ", luckyNumber)

	println()
	/*
		Dizilerin pointer yardımıyla referans olarak fonksiyonlara geçirilmesi mümkündür.
		Aşağıdaki kod parçasında points isimli int tipinden bir array tanımlıdır.
		plus fonksiyonu ile bu dizinin elemanları increment değeri kadar arttırılır.

		plus içerisinde dizi adresini gönderdiğimiz için, elemanlar üzerindeki değişim buradaki points dizisine de yansıyacaktır.
	*/
	points := [4]int{1, 2, 3, 4}
	plus(3, &points) // plus fonksiyonu ikinci parametrede bir pointer bekler. O nedenle & ile points'in adresini gönderiyoruz
	for _, p := range points {
		print(p, " ")
	}
	println()

}

func plus(increment int, numbers *[4]int) { //* ifadesi ile int tipinden dizi için bir pointer beklediğimizi belirtmiş oluyoruz
	for i := 0; i < len(numbers); i++ {
		numbers[i] += increment
		print(numbers[i], " ")
	}
	println()
}

/*
	Struct bir değer türü gibi davrnanış gösterir.
	Bu nedenle fonksiyona gelen Player nesnesinin fonksiyon kapsamında bir kopyası oluşturulur.
	Bu kopya üstünde yapılan puan artımı, main içerisindeki zeroOne'ın Point değeri üstünde bir etki yapmaz.
	Ancak decreasePoint fonksiyonunun kullanımı farklıdır.
*/
func increasePoint(p Player) {
	p.Point += 10
	println("Increase Point. ", &p, " ", p.Nickname, " ", p.Point)
}

// decrasePoint, pointer tipi kullanıyor. Player struct değişkenlerinin adresini bekliyor.
func decreasePoint(p *Player) {
	p.Point -= 2
	// Bu kez p değişkeni üstünden yapılan değişiklik, main fonksiyonundaki zeroOne değişkeninin değerlerini de etkileyecek.
	println("Decrease Point. ", p, " ", p.Nickname, " ", p.Point)
}

// struct türünden bir veri deseni tamamladık

// Player type
type Player struct {
	Nickname string
	Point    int
}
