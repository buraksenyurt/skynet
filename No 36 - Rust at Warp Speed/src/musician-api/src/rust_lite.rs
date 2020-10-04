/*
    products.json içerisindeki veriyi belleğe çekip bir vector'e parse eden ve 
    asenkron olarak farklı thread'lerin güvenli bir şekilde kullanabilmesine
    imkan sağlayan fonkisyonelliği tutan dosyamız.
*/
use crate::models::Product;
use serde_json::from_reader;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;

// Mutex<T> smart pointer nesnemiz Product türünden Vector taşıyacacak
// Arc = Atomic Referance Counting
pub type ProductDb = Arc<Mutex<Vec<Product>>>;

/*
    Json dosyasından veriyi yüklemek için kullanılan fonksiyonumuz

    Önce open fonksiyonu ile dosyayı açıyoruz.
    Eğer dosya içeriği başarılı şekilde okunduysa (Ok(json) durumu),
    veriyi JSON'dan ters serileştirip Product türünden vector nesnesine
    aktarıyoruz ve bunu kullanan Mutex'imizi örnekleyip geriye döndürüyoruz.

    Veriyi asenkron olarak gelecek Web API isteklerinin eş zamanlı kullanabilmesi 
    için Mutex<T> türünden yararlandık. Thread Safety olmasını da Arc tipinden faydalanarak sağladık.
*/
pub fn load() -> ProductDb {
    let file = File::open("./products.json");
    match file {
        Ok(json) => {
            let data = from_reader(json).unwrap();
            Arc::new(Mutex::new(data))
        }
        Err(_) => Arc::new(Mutex::new(Vec::new())),
    }
}
