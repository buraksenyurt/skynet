use crate::models::Product;
use serde_json::from_reader;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::Mutex;

// Mutex<T> smart pointer nesnemiz Product türünden Vector taşıyacacak
// Arc = Atomic Referance Counting
pub type product_db = Arc<Mutex<Vec<Product>>>;

/*
    Json dosyasından veriyi yüklemek için kullanılan fonksiyonumuz

    Önce open fonksiyonu ile dosyayı açıyoruz.
    Eğer dosya içeriği başarılı şekilde okunduysa (Ok(json) durumu),
    veriyi JSON olarak okuyup Product türünden vector'e dönüştürüyor ve 
    bunu kullanan Mutex'imizi örnekleyip geriye döndürüyoruz.

    Veriyi asenkron olarak gelecek Web API isteklerinin eş zamanlı kullanabilmesi 
    için Mutex<T> türünden yararlanıyoruz. Thread Safety olması içinde Arc tipinden
    yararlanmaktayız
*/
pub fn load() -> product_db {
    let file = File::open("./products.json");
    match file {
        Ok(json) => {
            let data = from_reader(json).unwrap();
            Arc::new(Mutex::new(data))
        }
        Err(_) => Arc::new(Mutex::new(Vec::new())),
    }
}
