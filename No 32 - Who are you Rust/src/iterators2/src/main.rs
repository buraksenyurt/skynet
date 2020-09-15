/*
    bir iterator kullanım örneği daha.
    Bu kez kendi veri yapımızın alanları üzerinde filtreleme işlemi gerçekleştiriyoruz.
*/

/*
    Birkaç klasik oyun bilgisini tutacak bir struct
*/
struct Game {
    name: String,
    year: u16,
    company: String,
    value: f32,
}

/*
    Yıla göre oyunları döndüren bir fonksiyon.
    Game türünden vector parametre olarak gelir, _year değerine göre filtreleme yapılır
    ve bu kritere uyan oyunlar geriye dönülür
*/
fn get_by_year(games: Vec<Game>, _year: u16) -> Vec<Game> {
    games.into_iter().filter(|g| g.year == _year).collect()
}

/*
    Örnek birkaç oyun bilgisi yüklediğimiz fonksiyon
*/
fn load_samples() -> Vec<Game> {
    vec![Game {
        name: String::from("Crazy Cars II"),
        year: 1988,
        company: String::from("Titus"),
        value: 1.5,
    },
    Game{
        name: String::from("Crazy Cars II"),
        year: 1988,
        company: String::from("Titus"),
        value: 1.5, 
    }]
}

fn main() {}
