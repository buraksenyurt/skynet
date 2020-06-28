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

	justsayit()
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
