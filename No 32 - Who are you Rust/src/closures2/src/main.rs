/*
    LINQ'teki Where, Find benzeri bir senaryoyu closure'ları kullanarak Rust üzerinde nasıl inşa edebiliriz bakmak istedim.

    Aşağıdaki senaryoda Person struct dizisi taşıyan bir vector'ümüz var. Bu vector üzerinde search isimli fonksiyon ile arama yapıyoruz.
    İşin sihri, arama fonksiyonunu kodu yazarken biz söylüyoruz ve bunu isimsiz fonksiyonla gerçekleştiriyoruz
*/

fn main() {
    let team = fill_players();

    println!("***Seviyesi 300 üstünde olan oyuncular***");
    let level_grater_than_300 = search(&team, |p: &Player| {
        return p.level >= 300;
    });
    for p in level_grater_than_300 {
        println!("{}[{}] (Avg:{})", p.nickname, p.level, p.average_point);
    }

    println!("\n***Sayı ortalaması 16 altında olan oyuncular***");
    let point_high = search(&team, |p: &Player| {
        return p.average_point < 16.0;
    });
    for p in point_high {
        println!("{}[{}] (Avg:{})", p.nickname, p.level, p.average_point);
    }

    println!("\n***Leykırs takımında olan oyuncular***");
    let point_high = search(&team, |p: &Player| {
        return p.team == "Leykırs";
    });
    for p in point_high {
        println!("{}[{}] (Avg:{})", p.nickname, p.level, p.average_point);
    }
}

// Oyuncu bilgilerini taşıyan struct
#[derive(Clone)] // search fonksiyonundaki for döngüsünde o anki player örneğinin bir klonunun oluşturulabilmesi için kullandığımız nitelik
struct Player {
    nickname: String,
    average_point: f32,
    level: i32,
    team: String,
}

/*
    generic search fonksiyonumuz.
    İlk parametrede Person tipinden bir vector alıyor ve ikinci parametre de F tipinden bir closure.
    Fn trait'i ile ifade edilen closure'un Person referansı aldığı ve geriye true veya false döndürmesi gerektiğini belirtiyoruz (where kısmı)
    Fonksiyon kendi içinde yeni bir vector oluşturuyor ve bunu geriye döndürüyor. Bu yeni vector içindeki Person nesnelerinin eklenme kriteri ise
    f fonksiyonu ile icra edilen koşul. Örneğin level'ı 300'den büyük olan oyuncuların çekilmesi gibi.
*/
fn search<F>(person_list: &Vec<Player>, f: F) -> Vec<Player>
where
    F: Fn(&Player) -> bool,
{
    let mut result: Vec<Player> = Vec::new();

    for p in person_list {
        if f(&p) {
            let plyr = p.clone();
            result.push(plyr);
        }
    }

    result
}

fn fill_players() -> Vec<Player> {
    let mut team: Vec<Player> = Vec::new();

    let mj = Player {
        nickname: String::from("M.J."),
        average_point: 32.50,
        level: 310,
        team: String::from("Şikago"),
    };
    let scoti = Player {
        nickname: String::from("Scoti pipin"),
        average_point: 15.5,
        level: 250,
        team: String::from("Şikago"),
    };
    let bird = Player {
        nickname: String::from("Leri börd"),
        average_point: 21.5,
        level: 320,
        team: String::from("Boston"),
    };
    let longle = Player {
        nickname: String::from("Luk longley"),
        average_point: 10.5,
        level: 100,
        team: String::from("Şikago"),
    };
    let conson = Player {
        nickname: String::from("Mecik Conson"),
        average_point: 28.95,
        level: 350,
        team: String::from("Leykırs"),
    };
    let doncic = Player {
        nickname: String::from("Luka doncic"),
        average_point: 22.34,
        level: 310,
        team: String::from("Dallas"),
    };
    let detler = Player {
        nickname: String::from("detler şiremğ"),
        average_point: 15.99,
        level: 280,
        team: String::from("Dallas"),
    };
    let karim = Player {
        nickname: String::from("karim abdul cabbar"),
        average_point: 21.99,
        level: 350,
        team: String::from("Leykırs"),
    };

    team.push(mj);
    team.push(scoti);
    team.push(bird);
    team.push(longle);
    team.push(conson);
    team.push(doncic);
    team.push(detler);
    team.push(karim);

    team
}
