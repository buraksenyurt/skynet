/*
    Product modeli

    serde kütüphanesini kullanarak serileştirme, ters serileştirme
    işlemlerini otomatize ediyoruz.

    Diğer yandan veritabanında saklamak isteyeceğimiz bir veri olacağından
    kopyalama işlemleri sırasında oluşabilecek borrowing sorunlarının da
    önüne geçiyoruz(Clone)
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub price: String,
}
