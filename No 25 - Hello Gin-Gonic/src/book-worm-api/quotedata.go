package main

/*
	Burada MongoDB ile ilgili CRUD Operasyonları yer alıyor.
*/

import (
	"context"
	"log"

	"go.mongodb.org/mongo-driver/bson/primitive"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

// getConn fonksiyonu ile mongodb bağlantı nesnesini ve context'i döndürüyoruz
func getConn() (*mongo.Client, context.Context) {

	/*
		NewClient nesnesini docker ile ayağa kalkan mongodb adresine göre oluşturuyoruz
		bir error olması durumu da aşağıya doğru sürekli kontrol edilmekte
	*/
	client, err := mongo.NewClient(options.Client().ApplyURI("mongodb://scoth:tiger@localhost:27017")) // ilgili bağlantı bilgisini kullanarak yeni bir client nesnesi oluşturuldu
	ctx := context.Background()

	err = client.Connect(ctx)
	if err != nil {
		log.Printf("Bağlanırken hata alındı: %v", err)
	}

	// Bağlantıyı kullanarak mongodb'yi ping'liyoruz
	err = client.Ping(ctx, nil)
	if err != nil {
		log.Printf("Ping yapılamadı: %v", err)
	}

	return client, ctx // geriye Context nesnesini de dönüyoruz. İzleyen metotlardaki defer kullanımlarına dikkat!
}

// AddQuote tahmin edileceği üzere bookworm veritabanındaki quotes koleksiyonuna yeni bir quote eklemek için kullanılıyor
func AddQuote(q *quote) (primitive.ObjectID, error) {
	log.Println("Add Quote")
	log.Println(q)

	client, ctx := getConn()
	defer client.Disconnect(ctx)   // AddQuote işleyişinin sonunda mongodb bağlantısının kapatılmasını garanti ediyoruz
	q.ID = primitive.NewObjectID() // eklenecej doküman için yeni bir Object ID üretiliyor ve parametre olarak gelen quote değişkenine yapıştırılıyor.

	// InsertOne ile q ile gelen quote değişkenini yolluyoruz. Eğer bir sorun olursa err parametresi hata bilgisini taşıyacaktır
	result, err := client.Database("bookworms").Collection("quotes").InsertOne(ctx, q)
	if err != nil { // Eğer hata olmuşsa bunu metottan geriye nil object ID ile birlikte dönüyoruz. Hatayı gin-gonic metotları değerlendirip uygun HTTP mesajını döndürecektir.
		log.Printf("Alıntı eklenmeye çalışırken hata oluştu %v", err)
		return primitive.NilObjectID, err
	}
	id := result.InsertedID.(primitive.ObjectID)
	return id, nil // Eğer sorun yoksa eklenen Object ID bilgisini dönüyor. Bu noktada hata olmadığı için ikinci output değişkeni nil olarak atanıyor
}

// GetQuotes metodu ile bookworm veritabanındaki quotes koleksiyonunda yer alan tüm dokümanları çekiyoruz
// Basit bir veri çekme metodu da olsa her ihtimale karşı hata kontrolümüz de var
func GetQuotes() ([]*quote, error) {
	var quotes []*quote // Döndüreceğimiz array

	client, ctx := getConn()     // MongoDb bağlantı bilgilerini aldık
	defer client.Disconnect(ctx) // Panik olsa da olmasa da metot tamalanırken Disconnect olalım

	db := client.Database("bookworms")            //veritabanı nesnesi
	collection := db.Collection("quotes")         // koleksiyon nesnesi
	cursor, err := collection.Find(ctx, bson.D{}) // quotes koleksiyonundaki tüm dokümanları çekmek için kullanılan fonksiyon
	if err != nil {                               // Find metodunun da error dönüşü var o yüzden kontrol etmekte fayda var
		return nil, err
	}
	defer cursor.Close(ctx)        // Eğer hata almadan geldiysek sonraki hata durumuna karşın Find ile açılan cursor'u kapattırır
	err = cursor.All(ctx, &quotes) // Koleksiyonu quotes'a alıyoruz
	if err != nil {                // All metodunun da error dönüşü var. Kontrol etmek iyi fikir
		log.Println(err)
		return nil, err
	}
	return quotes, nil // Her şey yolunda gittiyse ;)
}
