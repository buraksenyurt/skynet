package main

import (
	"math"
	"math/rand"
)

/*
	Sıkıcı belki ama veri tiplerine uğramadan yapamayız tabii.
*/

func main() {

	fullname := "James Bond" //string veri türü
	println(fullname)

	r := '©' // rune veri tipi. Karakterin unicode değerini tutar. Bu örnekte 0169'un int32 karşılığıdır
	println(r)

	pi := 3.1415 // float64 veri türü
	println(math.Pow(pi, 3))

	c1 := -3 + 7i // Kompleks sayı türü (sanal ve gerçel kökler float64 türünde tutulur)
	c2 := 9 + 5i
	c3 := c1 + c2 // Kompleks sayılarda toplama
	println(c3)

	var number = 4             // işaretsiz integer veri türü. Tabii girilen sayı büyüklüğüne göre değişebilir
	var e float32 = 3.23124    //32bit float veri tipi
	div := e / float32(number) // float32^yi float32'ye bölebileceğim için bir cast işlemi söz konusu
	println(div)

	c := byte('*') //byte veri türü uint18 alias'ı olarak ele alınır
	println(c)

	// 10 elemanlı bir dizi tanımı
	var numbers [10]int
	// Elemanlarına 0 ile 999 arasında rastgele sayı atarken toplamlarını da hesap eden kod parçası
	total := 0
	for i := 0; i < 10; i++ {
		numbers[i] = rand.Intn(999) //integer tipinde rastgele sayı üretir
		total += numbers[i]
	}
	println(total)

	// Sabit boyutlu diziyi ilk elemanlarını vererek tanımlamak
	names := [...]string{"red", "green", "blue", "black", "white"}
	for _, name := range names { // dizideki herbir kelimeyi gezdiğimiz for döngüsü
		println(name)
	}

	// Diziler değer türlü özellik gösterirler.
	// Aşağıdaki atama sonrası names dizisinin bir kopyası oluşur
	// ama bellekte ayrık tutulurlar
	// Birinde yapılan değişiklik diğerini etkilemez
	copyofnames := names
	copyofnames[0] = "pink"
	println("Kopya dizideki 0ncı eleman->", copyofnames[0])
	println("Kaynak dizideki 0ncı eleman->", names[0])

	// slice ve map kullanımo örnekleri gelecek
}
