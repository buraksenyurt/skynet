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

	// Dizi veri türü

	// 10 elemanlı bir dizi tanımı
	var numbers [10]int
	// Elemanlarına 0 ile 999 arasında rastgele sayı atarken toplamlarını da hesap eden kod parçası
	total := 0
	for i := 0; i < 10; i++ {
		numbers[i] = rand.Intn(999) //integer tipinde rastgele sayı üretir
		total += numbers[i]
	}
	println(total)

	/*
		Sabit boyutlu diziyi ilk elemanlarını vererek tanımlamak.
		... operatörünün kullanıldığı yerlerden birisi de burasıdır.
		Diğer kullanım şekillerine de bakın
	*/
	names := [...]string{"red", "green", "blue", "black", "white"}
	for _, name := range names { // dizideki herbir kelimeyi gezdiğimiz for döngüsü
		println(name)
	}

	/*
		Diziler değer türlü özellik gösterirler.
		Aşağıdaki atama sonrası names dizisinin bir kopyası oluşur ama bellekte ayrık tutulurlar.
		Birinde yapılan değişiklik diğerini etkilemez.
	*/
	copyofnames := names
	copyofnames[0] = "pink"
	println("Kopya dizideki 0ncı eleman->", copyofnames[0])
	println("Kaynak dizideki 0ncı eleman->", names[0])

	/*
		slice veri türü.

		slice dizilerin aksine sabit boyutlu tanımlanmak zorunda değildir. Boyutu dinamik olarak artar.
		Yine diziler gibi value type değil reference type'tır. Yani kopyalama sonrası örnekler birbirlerini etkiler
	*/
	points := []float32{3.2, 1.5, 7.8, 6.33}
	println("Slice eleman sayısı ", len(points))                // slice'ın güncel eleman sayısı
	points = append(points, 4.5, 3.6)                           //built-in append fonksiyonunu kullanarak ilk parametre olarak gelen slice'e bir veya daha fazla eleman eklenebilir
	println("append sonrası slice eleman sayısı ", len(points)) // append çağrısı sonrası slice'ın yeni eleman sayısı
	newPoints := []float32{1.1, 1.2, 1.3}                       // başka bir slice
	points = append(points, newPoints...)                       // bir slice'a başka bir slice'ı da ekleyebiliriz. Sondaki elipses
	// slice nesnelerini ... operatörü ile variadic fonksiyonlara parametre olarak da gönderebiliriz
	println("... operatörü ile slice toplamı ", sumofall(points...))

	/*
		maps türü.

		Çok basit anlamda key:value çiftlerinin tutulduğu veri türü gibi düşünüleblir.
		hash ve dictionary tadındadır.
		Dinamik olarak boyutlandırılabilir
	*/
	serverCodes := map[int](string){100: "ISTCMDP01", 101: "ISTCMDD02"}
	serverCodes[201] = "MNCHNPRD1002" // Yeni key:value çifti ekleme
	serverCodes[301] = "TKYOPREPROD98"
	writeall(serverCodes)
	serverCodes[100] = "ISTCMDPRODO1" // Güncelleme
	delete(serverCodes, 101)          // silme
	println("")
	writeall(serverCodes)
}

func writeall(servers map[int](string)) {
	for k, v := range servers {
		println(k, ":", v)
	}
}

func sumofall(numbers ...float32) (result float32) {
	for _, n := range numbers {
		result += n
	}
	return
}
