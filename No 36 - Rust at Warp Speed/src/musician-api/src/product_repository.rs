use crate::models::Product;
use crate::rust_lite::ProductDb;
use std::convert::Infallible;

/*
    Ürün listesini Thread-safe döndüren fonksiyon
    rust_lite paketindeki Product_Db'yi kullanıyor ki O da products.json dosyası ile beslenmekte
*/
pub async fn get_products(db: ProductDb) -> Result<impl warp::Reply, Infallible> {
    let products = db.lock().await; // Arc klonlandı ve thread-safety sağlandı
    let products: Vec<Product> = products.clone(); // Mutex içindeki veriyi de klonladık
                                                   // json formatlı olarak geriye döndürdük
    Ok(warp::reply::json(&products))
}
