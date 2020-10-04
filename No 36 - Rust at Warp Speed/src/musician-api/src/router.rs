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
    get_all(db.clone())
}

fn get_all(
    db: ProductDb,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("products")
        .and(warp::get())
        .and(with_db(db))
        .and_then(product_repository::get_products)
}

/*
    Veritabanı referansını (ki bu örnekte Product_Db nesnesi) router tarafına referans olarak 
    paslayabilmek için kullanılan yardımcı fonksiyon.
*/
fn with_db(db: ProductDb) -> impl Filter<Extract = (ProductDb,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}