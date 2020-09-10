/*
    lifetime kavramı ile ilgili başka bir senaryo ile devam edelim.

    find_winner isimli fonksiyonu Player tipinden iki referansı parametre olarak alır ve geriye yine bir Player referansı döndürür.
    find_winner fonksiyonunun parametre olarak gelen Player değişkenlerini sahiplenmesini istemiyoruz. Bu nedenle referans olarak geçtik.
    Lakin Rust derleyicisi ve özellikle Borrow Checker mekanizması bir kafa karışıklığı yaşayacaktır.
    p1'in mi yoksa p2'nin mi geriye döneceği belli değildir. Bu durumda find_winner'dan dönecek Player referansının(p1 veya p2 olabilir) ne kadar süre yaşaması gerektiği belli değildir.
    p1'inki kadar mı ömrü olmalıdır, yoksa p2'nin kadar mı?
    Bu durum derleyicinin "explicit lifetime required in the type of `p2`" benzeri bir hata vermesi ile devam eder.

    Olayın önüne geçmek için generic lifetime parametreleri kullanmamız gerekiyor. Böylece referanslar arası yaşam süreleri için ilişki kurabiliriz
*/

struct Player {
    nick_name: String,
    total_point: i32,
}

// // lifetime hatası veren versiyon
// fn find_winner(p1: &Player, p2: &Player) -> &Player {
//     if p1.total_point > p2.total_point {
//         return p1;
//     } else {
//         return p2;
//     }
// }

/*
    'l lifetime'ın adıdır ve &'l Player, Player referansı için 'l kadarlık bir yaşam ömrü belirttiğimizi ifade eder.
    Bir başka deyişle referansın yaşam ömrünü açık bir şekilde belirtmiş oluruz.

    Bu yeni sürümde p1, p2 ve geriye dönen Player dahil olmak üzere 3 referansta aynı yaşam sürelerine sahiptir.
*/
fn find_winner<'l>(p1: &'l Player, p2: &'l Player) -> &'l Player {
    if p1.total_point > p2.total_point {
        return p1;
    } else {
        return p2;
    }
}

fn main() {
    let gustavo = Player {
        nick_name: String::from("Gustavo"),
        total_point: 18,
    };
    let mikel = Player {
        nick_name: String::from("Mikel"),
        total_point: 17,
    };
    let winner = find_winner(&gustavo, &mikel);
    println!("Kazanan `{}`", winner.nick_name);

    /*
        #2 Aşağıda yine enteresan bir yaşam ömrü sorunsalı yer almaktadır.
        schumi ve race_winner iç scope dışında tanımlıdır. Toplam puanlara baktığımızda kazanan schumi'dir ve dolayısıyla,
        #İlginç yazan yerde race_winner, schumi'nin referansını taşıyacağı için bir sorun olmaması beklenmektedir.
        Ne var ki find_winner fonksiyonu parametreleri ve geriye dönen Player referansı için aynı yaşam süresini beklemektedir.
        Koda göre #İlkÇıkış noktasında hakinen'in ömrü dolmaktadır. Yani schumi, hakinen ve kazanan için aynı yaşam döngüsü kuralı bozulmuştur.
        Bu nedenle derleyici aşağıdaki kod parçası için `hakinen` does not live long enough diyecektir.
    */
    let schumi = Player {
        nick_name: String::from("Schumi"),
        total_point: 77,
    };
    let race_winner;
    {
        let hakinen = Player {
            nick_name: String::from("hakinen"),
            total_point: 60,
        };
        race_winner = find_winner(&schumi, &hakinen);
    } // #İlkÇıkış
    println!("Yarışın kazananı {}", race_winner.nick_name); // #İlginç
}

struct Game<'l> {
    // color_name: &str, // struct türünde referans türlü alanlarda kullanabiliriz ancak bu şekilde değil. lifetime bildirimi ile kullanabiliriz
    color_name: &'l str,
    max_player: i32,
}
