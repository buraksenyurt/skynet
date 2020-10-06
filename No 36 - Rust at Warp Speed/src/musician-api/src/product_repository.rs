use crate::models::Product;
use crate::rust_lite::ProductDb;
use std::convert::Infallible;

use warp::{self, http::StatusCode};

/*
    Ürün listesini Thread-safe döndüren fonksiyon
    rust_lite paketindeki Product_Db'yi kullanıyor ki O da products.json dosyası ile beslenmekte
*/
pub async fn get_all(db: ProductDb) -> Result<impl warp::Reply, Infallible> {
    println!("get_products fonksiyonu çağrıldı");

    let products = db.lock().await; // Arc klonlandı ve thread-safety sağlandı
    let products: Vec<Product> = products.clone(); // Mutex içindeki veriyi de klonladık
                                                   // json formatlı olarak geriye döndürdük
    Ok(warp::reply::json(&products))
}

/*
    Id bilgisine göre ürün bilgisi yine Thread-Safe döndüren fonksiyon.
    Bu fonksiyonu servise gelen talepleri karşılayan router kullanıyor.
*/
pub async fn get_by_id(id: String, db: ProductDb) -> Result<Box<dyn warp::Reply>, Infallible> {
    println!("get_by_id fonksiyonu çağrıldı");

    // Önce db nesnesini tutan Mutex thread-safe klonlanır
    let products = db.lock().await;
    // Amelece olacak ama tüm vector nesnelerini bir iterasyon ile dolaşıyoruz
    for p in products.iter() {
        // parametre olarak gelen id'yi bulursak
        if p.id == id {
            // Bulunan vector satırının json formatına dönüştürülmüş halinin
            // Heap'e çekilmiş bir versiyonunu döndürüyoruz
            return Ok(Box::new(warp::reply::json(&p)));
        }
    }

    // Eğer kayıt bulunamazsa HTTO 404 Not Found durumunu döndüreceğiz
    Ok(Box::new(StatusCode::NOT_FOUND))
}

/*
    Yeni bir ürünün eklenmesi işini üstlenen fonksiyonumuz.
    Router tarafında yeni bir ürün oluşturmak için gelecek POST talebi bu fonksiyona inecek
*/
pub async fn create(payload: Product, db: ProductDb) -> Result<impl warp::Reply, Infallible> {
    println!(
        "Create operasyonuna gelen içerik\n{} {} {}",
        payload.id, payload.title, payload.price
    );
    let mut products = db.lock().await;
    products.push(payload); // vector'e gelen ürünü ekliyoruz
    Ok(StatusCode::CREATED) // HTTP 201 döndürüyoruz
}
