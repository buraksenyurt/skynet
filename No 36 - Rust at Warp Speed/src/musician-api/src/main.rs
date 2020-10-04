use warp;
mod models;
mod product_repository;
mod router;
mod rust_lite;

/*
    Eş zamanlı olarak talep karşılayacak olan main fonksiyonumuz.
    async ile işaretlenmesinin sebebi de bu.
    veritabanını (Aslında json dosya içeriğini belleğe alıp kullandık)
    ve HTTP talep yönlendiricisini örnekledikten sonra,
    warp::serve fonksiyonu ile web sunucusunu localhost:5555 portundan etkinleştirdik.
    router.rs içindeki talimatlara göre talepleri yollayabiliriz.
*/
#[tokio::main]
async fn main() {
    let db = rust_lite::load();
    let product_router = router::setup(db);

    warp::serve(product_router)
        .run(([127, 0, 0, 1], 5555))
        .await;
}
