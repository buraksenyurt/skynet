package main

/*
	Örnekte birkaç fonksiyon kullanım örneği yer alıyor.
	calculate özellikle birden fazla parametre döndürebilme kabiliyeti ile dikkate değer
*/

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

func main() {
	var someword string            //string türde değişken tanımı
	someword = "How are you body!" //değer ataması

	// fonksiyonun çağrılması
	smart := beautify(someword) // := ile reversed isimli değişkenin tipinin tahmini Go'ya bırakılır.

	// Println ile çıktı yazdırılır
	fmt.Println("Cümle->", someword, "\nTersi->", smart)

	// Görüldüğü gibi calculate fonksiyonunun 4 adet çıktısını eşitliğin sol tarafındaki 4 değişkene tek satırda atayabildik
	t, b, c, f := calculate(math.Pi, math.E)
	fmt.Println("Sum=", t, "Div=", b, "Mul=", c, "Dif=", f)

	// variadic fonksiyonun kullanımı
	total := sumofall(1, 1, 2, 3, 5, 8, 13, 21)
	println(total)

	/*
		Aşağıda bir isimsiz fonskiyon örneği yer alıyor.
		Bu isimsiz fonksiyon Sum isimli değişkene atanmış durumda.
		Fonksiyon kod bloğu içerisindeki x değişkeni ile kendisine parametre olarak gelen y değişkenini kullanıyor.
		Fonksiyonun kendi kod bloğu dışındaki bir değişkene (x) eriştiğini de görüyoruz. Bu Closure olarak isimlendirilen durum.
		Bu isimsiz fonksiyonu kullanmanın tek yolu sum değişkenini çağırmak. println içindeki kullanımı çok alışılmadık değil.
		İsimsiz fonksiyoları ifade içerisinde tanımlayıp kullanabiliriz de. 56ncı satırdaki kullanıma dikkat edelim.
	*/
	x := 0

	sum := func(y int) (total int) {
		total = x + y
		return
	}
	fmt.Printf("sum değişkeninin tipi %T", sum)
	println()
	x = 1
	println("1+2=", sum(2))
	x = 4
	println("4+5=", sum(5))

	// Fonksiyon tanımını ve kullanımını ifade içerisinde gerçekleştirdik. Bu da isimsiz(Anonymous) bir fonksiyon kullanımı örneği aslında.
	println("1...10 arasındaki sayıların toplamı=", func(a, b int) int {
		ttl := 0
		for i := a; i < b; i++ {
			ttl += i
		}
		return ttl
	}(1, 10), "şeklindedir.")

	/*
		Higher Order Functions...
		Bir fonksiyonu başka bir fonksiyonu parametre olarak atayabilir ve
		hatta fonksiyondan fonksiyon döndürebiliriz.
		Aşağıdaki isContainH ve isContainGo fonksiyonları, searchAny fonksiyonuna parametre olarak atanıyorlar.
		searchAny fonksiyonuna parametre olarak gelen string slice içerisindeki her bir eleman için,
		f isimli fonksiyon tetikleniyor.
	*/
	isContainH := func(word string) bool {
		return strings.ContainsAny(word, "H")
	}
	searchAny([]string{"Die Hard III", "Rambo II", "Hobbits I", "Specialist"}, isContainH)

	isContainGo := func(word string) bool {
		return strings.ContainsAny(word, "W")
	}
	searchAny([]string{"Practial Go, Addison Wesley Publishing", "C# Cookbook", "Wild Wild West", "Shrek"}, isContainGo)

	/*
		Bu sefer geriye fonksiyon döndüren bir fonksiyon kullanıyoruz.
		Bu da High Order Function özelliğidir.
	*/
	tf := timeFactory("Cuma")
	println(tf("Perşembe", "Cumartesi"))

	justsayit()
}

/*searchAny fonksiyonu ilk parametre olarak string türünden bir slice alıyor.
İkinci parametre ise isimsiz bir fonksiyon. values içerisindeki herbir string değeri bu isimsiz fonksiyona parametre olarak gönderiyoruz.
*/
func searchAny(values []string, f func(wrd string) bool) {
	for _, v := range values {
		result := f(v)
		if result {
			println(v, "-> Aranan kritere uygun")
		}
	}
}

// word isimli string türde paramtre alıp string türden çıktı veren bir fonksiyon
func beautify(word string) (result string) {

	var newWord string

	// parametre olarak gelen string'in herbir karakterinde dolaşacağız
	// Normalde _ yerine i gibi bir değişken adı yazılıp index değerine de ulaşılabilir
	for _, c := range word {
		newWord += " " + string(c) // += ile herbir harfin önceki newWord'e aralara boşluk konularak eklenmesi sağlanıyor
	}

	return newWord
}

/*
timeFactory isimli fonksiyon da bir Higher Order Function olarak nitelendirilebilir.
Nitekim geriye fonksiyon döndürmektedir.
Fonksiyon imzasında dönüş fonksiyonunun nasıl olacağı belirtilir
Blok içerisinde dikkat edileceği üzere func ile dönüş fonksiyonu yazılmaktadır.
*/
func timeFactory(currentDay string) func(before, after string) string {
	return func(before, after string) string {
		return fmt.Sprintf("%s %s %s", before, currentDay, after)
	}
}

// Bu da birden fazla değer döndürebilen fonksiyonlara güzel bir örnek
// Fonksiyon iki parametre alıp 4 sonuç döndürüyor
func calculate(x, y float32) (sum, div, mul, dif float32) {
	return x + y, x / y, x * y, x - y // ve burada görüldüğü gibi return sonrası kaç tane çıktısı varsa döndürülebiliyor
}

// parametresiz ve geriye bir şey döndürmeyen bir fonkisyonu
func justsayit() {
	// Terminal girişlerini okumak için bir nesne oluşturduk
	reader := bufio.NewReader(os.Stdin)
	for { //sonsuz bir döngü
		fmt.Print("...")
		command, _ := reader.ReadString('\n')            // alt satıra geçince komutu almak istediğimizden \n parametresi söz konusu
		command = strings.Replace(command, "\n", "", -1) // tabii bu durumda da fazladan CRLF ortaya çıkacak. Onu LF'ye dönüştürüyoruz
		println(command)
		if command == "quit" { //quit yazarsa çıkabiliriz
			fmt.Println("Goodbye Crow")
			break // for döngüsünden çıkartır
		}
	}
}

/*
	Variadic fonksiyon örneği. numbers tanımından sonra gelen
	... operatörü, fonksiyona istenen sayıda int tipini gönderebileceğimizi ifade eder.
	... operatörünün bir diğer kullanım şekli append ile slice'a yeni değerler eklenmesinde
	karşımıza çıkmaktadır
*/
func sumofall(numbers ...int) (result int) {

	for _, n := range numbers {
		result += n
	}

	return // metodun dönüş parametresini isimlendirdiğimizde sadece return diyebiliriz
}
