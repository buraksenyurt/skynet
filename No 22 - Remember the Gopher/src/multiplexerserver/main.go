package main

import (
	"fmt"
	"net/http"
)

/*
	Bu örnekte lighthttpserver örneğinden farklı olarak web sunucusu için multiplexer yapısı kullanılıyor.
	Bu kullanımda talepleri ayırt etmek daha kolay
*/

func main() {

	// Burada / adresine gelen talepleri aldık
	http.HandleFunc("/", func(res http.ResponseWriter, req *http.Request) {
		// Basit bir HTML çıktısı üretip döndürüyoruz
		// Bir HTML dosya içeriğini okuyup basabiliriz de.
		header := res.Header()
		header.Set("Content-Type", "text/html")
		res.WriteHeader(http.StatusOK)
		fmt.Fprint(res, `<h1>Merhaba dostum</h1>`)
	})

	// Burada /api/person adresine gelen talepleri değerlendirdik
	http.HandleFunc("/api/person", func(res http.ResponseWriter, req *http.Request) {
		header := res.Header()
		header.Set("Content-Type", "application/json")
		res.WriteHeader(http.StatusOK)
		fmt.Fprint(res, `{"nickName":"m.j.","level":"god mod on","number":23}`)
	})

	/*
		ListenAndServe'ün ilk parametresi ile localhost üzerinde 5557 nolu porttan dinleme yapacağımızı belirttik
		İkinci parametreye nil değer atadığımız için go http.DefaultServeMux fonksiyonunu kullanılır ve bu da
		bir ServeMux örneğidir. Yukarudaki HandlerFunc tanımlamaları bu sayede devreye girer
	*/
	err := http.ListenAndServe(":5557", nil)
	fmt.Println(err)
}
