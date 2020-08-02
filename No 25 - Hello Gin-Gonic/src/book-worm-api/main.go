package main

// gin-gonic modülünü ekledik
import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	router := gin.Default()      // gin nesnesini örnekledik
	api := router.Group("/api")  //ve api'nin
	v1 := api.Group("/v1")       // v1 sürümü için
	quotes := v1.Group("/quote") //quotes isimli bir route tanımladık

	// Bu route için kök adrese HTTP Get talebi gelirse tüm alıntıları listeyecek operasyonu çalıştırıyoruz
	quotes.GET("/", func(c *gin.Context) {
		// mongodb ile iletişim kuran quotedata içerisindeki ilgili metodu çağırdık
		var quoteList, err = GetQuotes()
		// Her ihtimale karşı listeyi çeken metot bir hata döndürmüş mü bakalım
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"msg": err})
			return
		}
		c.JSON(http.StatusOK, gin.H{"quotes": quoteList}) // HTTP 200 OK ile birlikte çekilen listeyi geri yolluyoruz.
	})

	// Kök adrese bir Post talebi gelirse bu sefer yeni bir alıntının eklenmesini sağlıyoruz
	quotes.POST("/", func(c *gin.Context) {
		var q quote
		// Gelen JSON içeriğinin quote yapısına eşlenip eşlenemediği kontrol ediliyor
		if err := c.ShouldBindJSON(&q); err != nil {
			log.Print(err)
			c.JSON(http.StatusBadRequest, gin.H{"msg": err}) // Eğer JSON içeriğinde sıkıntı varsa hata mesajı ile birlikte geriye HTTP 400 Bad Request dönüyoruz
			return
		}
		id, err := AddQuote(&q) // Add metodu ile eklemeyi gerçekleştiriyoruz
		if err != nil {         // eğer hata varsa bunu da değerlendirip geriye uygun bir HTTP durum kodu ile döndürüyoruz
			c.JSON(http.StatusInternalServerError, gin.H{"msg": err})
			return
		}
		q.ID = id
		c.JSON(http.StatusOK, gin.H{"added": q}) // Her şey yolundaysa HTTP 200 Ok
	})

	_ = router.Run(":5003") // sunucuyu 5003 numaralı porttan hizmete açtık
}
