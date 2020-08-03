package main

// gin-gonic modülünü ekledik
// Ayrıca Swagger desteği için gin-swagger modülü de eklendi
import (
	"log"
	"net/http"

	ginSwagger "github.com/swaggo/gin-swagger"
	"github.com/swaggo/gin-swagger/swaggerFiles"

	_ "book-worm-api/docs" //swag init ile eklenen dokümanın bildirimi
	// Bunu eklememin sebebi çalışma zamanında swagger UI'a gidince aldığım Failed to load spec. hatası

	"github.com/gin-gonic/gin"
)

// @title BookWorm Swagger API
// @version 1.0
// @description Servis kullanım rehberidir
// @termsOfService http://swagger.io/terms/

// @contact.name Burak Selim Şenyurt
// @contact.email selim@buraksenyurt.com
// @contact.url https://www.buraksenyurt.com

// @BasePath /api/v1

// @host localhost:5003
func main() {
	router := gin.Default()      // gin nesnesini örnekledik
	api := router.Group("/api")  //ve api'nin
	v1 := api.Group("/v1")       // v1 sürümü için
	quotes := v1.Group("/quote") //quotes isimli bir route tanımladık

	// Bu route için kök adrese HTTP Get talebi gelirse tüm alıntıları listeyecek operasyonu çalıştırıyoruz
	quotes.GET("/", GetAll)

	// Kök adrese bir Post talebi gelirse bu sefer yeni bir alıntının eklenmesini sağlıyoruz
	quotes.POST("/", Create)

	/*
		Swagger API dokümantasyon desteği için ekledik.
		metot başlarında yer alan yorum satırları da Swagger UI tarafında değerlenecek
	*/
	router.GET("/swagger/*any", ginSwagger.WrapHandler(swaggerFiles.Handler))

	_ = router.Run(":5003") // sunucuyu 5003 numaralı porttan hizmete açtık
}

/*
	@Summary operasyon için açıklama kısmıdır.
	@Produce ve @Accept ile HTTP operasyonun hangi tür içerikle çalıştığını söylüyoruz ki örnekte json.
	@Param kısmında Body içinden quote tipinden bir değişken beklediğimizi ve zorunlu olduğunu ifade ederiz
	@Sucess ile operasyon başarılı ise HTTP 200 dönüldüğünü ifade ederiz
	@Failure kısımlarında metottan hangi tür HTTP hatalarının dönebileceğini ifade ediyoruz
	@Router kısmında operasyon adresini HTTP POST metodu ile tetiklendiğini ifade ediyoruz
*/

// Create godoc
// @Summary Yeni bir kitap alıntısı ekler
// @Produce json
// @Accept json
// @Param quote body quote true "Alıntı Bilgileri"
// @Success 200
// @Failure 400
// @Failure 500
// @Router /quote/ [post]
func Create(c *gin.Context) {
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
}

/*
	{array} ile bir quote listesinin döneceğini belirtiyoruz
*/

// GetAll godoc
// @Summary Tüm kitap alıntılarını döndürür
// @Produce json
// @Success 200 {array} quote
// @Failure 500
// @Router /quote/ [get]
func GetAll(c *gin.Context) {
	// mongodb ile iletişim kuran quotedata içerisindeki ilgili metodu çağırdık
	var quoteList, err = GetQuotes()
	// Her ihtimale karşı listeyi çeken metot bir hata döndürmüş mü bakalım
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"msg": err})
		return
	}
	c.JSON(http.StatusOK, gin.H{"quotes": quoteList}) // HTTP 200 OK ile birlikte çekilen listeyi geri yolluyoruz.
}
