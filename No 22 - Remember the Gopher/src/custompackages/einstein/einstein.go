package einstein

import "fmt"

/*
	Sum fonksiyonu ile ilgili küçük bir not.
	Baş harfi büyük olan paket fonksiyonları veya değişkenleri dışarıya açıktır.
	Sum'ı sum şeklinde yazdığımızda main paketinde kullanamadığımızı görürüz.
*/
func Sum(x int, y int) int {
	total := x + y
	return total
}

//init fonksiyonu, einstein isimli paket oluşturulduğunda çalışır. (C# taki constructor gibi düşün)
func init() {
	fmt.Println("einstein paketi oluşturuldu")
}

/*
	einstein klasörü altında yer alan einstein.go, custompackages modülü içerisinde yer alan bir pakettir.
	Bu paketin dışarıya açık olan(baş harfi büyük) örnek bir fonksiyonunu main içerisinde kullanıyoruz.

	Go dilinde bu klasörlendirme mantığı ile fonksiyonları kendi alanlarına ait paketlerde toplayıp kullanabiliriz.

	Bir kural olarak Paket adı ile klasör adı aynı olmalıdır. Örneğimizde custompackages bir modüldür.
	einstein ise bu modül içerisinde tanımlanmış bir pakettir.
*/
