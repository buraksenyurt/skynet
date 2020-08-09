/*
	Örnekte basit bir http server kodu yer almaktadır
*/
package main

import (
	"fmt"
	"io"
	"net/http"
)

func main() {
	h := ozHTTPHandler{}
	/*
		ListenAndServe ilk parametre ile belirtilen adrese (örnekte localhost:1923) gelen talepleri dinler
		ve bu talepleri işletmek üzere ikinci parametre ile belirtilen nesneye aktarır.

		ListenAndServe metodunun ikinci parametresi HttpHandler isimli arayüzü(interface) tipini kullanır.

		Örnekteki ozHttpHandler isimli struct bu arayüzün tanımladığı ServeHTTP isimli metodu uygulamaktadır.

	*/
	err := http.ListenAndServe(":5556", h)
	fmt.Println(err)
}

type ozHTTPHandler struct{}

/*
	ServeHttp gelen talebe döneceğimiz cevap için kullanabileceğimiz fonksiyon.
*/
func (h ozHTTPHandler) ServeHTTP(res http.ResponseWriter, req *http.Request) {
	data := []byte("Basit bir Web Sunucusu Doğuyor!")
	res.Write(data) // Yukarıdaki binary içeriği ResponseWriter'ın Write metodu ile istemciye basıyoruz
	// io.WriteString metodu ile bir string içeriği res değişkeni ile belirtilen httpResponse'a basabiliriz.
	// Kullanıcının sunucu üstünde talep ettiği adresi geri basıyoruz.
	io.WriteString(res, "\nRequest URL: "+req.URL.String())
	// req.URL ile gelen talebe ait bilgileri yakalayıp ona uygun davranmak da mümkün.
	// index.html talebi gelmişse böyle yap, .css talebi gelmişse şöyle yap gibi.
}
