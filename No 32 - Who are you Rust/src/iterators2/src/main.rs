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
    publisher: String,
    value: f32,
    platform: Platform,
}

/*
    Oyunun hangi console'da oynandığı bilgisini de bir enum ile tutalım.
    Bu arada game_by_platform fonksiyonundaki == operatörünü kullanabilmek için PartialEq niteliğini kullanıyoruz
*/
#[derive(PartialEq)]
enum Platform {
    Commodore64,
    Atari2600,
    Atari5200,
}

/*
    Belli bir yıldan önceki oyunları döndüren bir fonksiyon.
    Game türünden vector parametre olarak gelir, _year değerine göre filtreleme yapılır
    ve bu kritere uyan oyunlar geriye dönülür
*/
fn less_than_year(games: Vec<Game>, _year: u16) -> Vec<Game> {
    games.into_iter().filter(|g| g.year <= _year).collect()
}

/*
    Belli bir platform için yazılmış oyunların bulunması
*/
fn games_by_platform(games: Vec<Game>, _platform: Platform) -> Vec<Game> {
    games
        .into_iter()
        .filter(|g| g.platform == _platform)
        .collect()
}

/*
    Örnek birkaç oyun bilgisi yüklediğimiz fonksiyon
*/
fn load_samples() -> Vec<Game> {
    vec![
        Game {
            name: String::from("Crazy Cars II"),
            year: 1988,
            publisher: String::from("Titus"),
            value: 1.5,
            platform: Platform::Commodore64,
        },
        Game {
            name: String::from("1942"),
            year: 1986,
            publisher: String::from("Elit"),
            value: 2.85,
            platform: Platform::Commodore64,
        },
        Game {
            name: String::from("Pitstop II"),
            year: 1984,
            publisher: String::from("Epyx"),
            value: 0.55,
            platform: Platform::Commodore64,
        },
        Game {
            name: String::from("The Last Ninja"),
            year: 1987,
            publisher: String::from("System 3"),
            value: 1.49,
            platform: Platform::Commodore64,
        },
        Game {
            name: String::from("Spy Hunter"),
            year: 1983,
            publisher: String::from("US Gold"),
            value: 2.40,
            platform: Platform::Commodore64,
        },
        Game {
            name: String::from("3-D Tic Tac Toe"),
            year: 1980,
            publisher: String::from("Atari"),
            value: 6.75,
            platform: Platform::Atari2600,
        },
        Game {
            name: String::from("Asteroids"),
            year: 1981,
            publisher: String::from("Atari"),
            value: 6.70,
            platform: Platform::Atari2600,
        },
        Game {
            name: String::from("Gremlins"),
            year: 1986,
            publisher: String::from("Atari"),
            value: 2.75,
            platform: Platform::Atari5200,
        },
        Game {
            name: String::from("Mario Bros."),
            year: 1988,
            publisher: String::from("Nintendo"),
            value: 9.85,
            platform: Platform::Atari5200,
        },
    ]
}

fn main() {}
