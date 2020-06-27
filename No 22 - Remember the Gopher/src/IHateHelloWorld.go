/*
	Hello World demek adettendir.
	Bu arada toplu bir yorum satırı nasıl eklenir,
	onu da görmüş oluyoruz.

	Her go kod dosyası aslında bir pakettir
	main özel bir paket adıdır ve söz konusu dosyanın çalıştırılabilir (executable) olduğunu ifade eder
*/

package main

// Kullanılacak diğer paketleri import bildirimi altında belirtiriz
import (
	f "fmt" //GO'nun standart kütüphanelerindendir. Baştaki f ile bu pakete bir alias verdik. Kullanımına dikkat
)

// Çalıştırılabilir dosyaların giriş noktası main fonksiyonudur
func main() {
	for i := 0; i < 3; i++ {
		f.Println("Gopher!")
	}
}
