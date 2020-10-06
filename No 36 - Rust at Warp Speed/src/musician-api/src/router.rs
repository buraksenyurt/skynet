/*
    Burası router işlemlerini yönettiğimiz yer
    Mesela doğrudan /products adresine gelecek HTTP Get taleplerine karşılık get_all'un çalışmasını sağlıyoruz.
    Yönlendirme adresleri için warp'un path fonksiyonundan yararlanıyoruz.
    HTTP nin hangi metodunu karşılayacağımız warp::get, warp::post, warp::put gibi çağrılarla belirleniyor.
*/
use std::convert::Infallible;
use warp::{self, Filter};

// use crate::models::Product;
use crate::product_repository;
use crate::rust_lite::ProductDb;

pub fn setup(
    db: ProductDb,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    /*
        Sırasıyla HTTP taleplerini ele alacak fonksiyonlar bildiriliyor.
        (get_all, get_by_id...)
        Bu fonksiyonlar eş zamanlı gelecek istemci taleplerini işlerken db nesnesinin
        (ProductDb) thread-safe klonlanmış bir versiyonlarını parametre olarak alıyorlar.
    */
    get_by_id(db.clone()).or(get_all(db))
    //get_all(db.clone()).or(get_by_id(db)) // Bomba soru için eklendi
}

// Burası /products için HTTP Get talebi geldiğinde çalışacak olan fonksiyon
fn get_all(
    db: ProductDb,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("products")
        .and(warp::get())
        .and(warp::any().map(move || db.clone())) //Veritabanı referansını (ki bu örnekte Product_Db nesnesi) router tarafına referans olarak paslayabilmek için kullanılan yardımcı fonksiyon.
        .and_then(product_repository::get_products)
}

/*
    URL'den gelen id değerine göre ürün bilgisi getirecek fonksiyonumuz.
    products/{id} şeklinde bir map söz konusudur.
*/
fn get_by_id(
    db: ProductDb,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // path! makrosu URL tarafında parametre kullanımını kolaylaştırır
    warp::path!("products" / String)
        .and(warp::get())
        .and(warp::any().map(move || db.clone())) //with_db fonksiyonelliğini bu şekilde closure olarak da kullanabiliriz
        .and_then(product_repository::get_by_id)
}
