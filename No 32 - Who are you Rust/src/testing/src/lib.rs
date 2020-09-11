/*
    Basit Unit Test yazmak

    cargo new testing --lib
    terminal komutu ile bir kütüphane açtığımızda içerisine otomatik olarak tests isimli bir modül açılır.

    test etmek için terminalden

    cargo test

    komutunu çalıştırmak yeterlidir.

    Test fonksiyonları fail durumuna düştüğünde Rust çalışma zamanı bir panik havası estirir.
*/
#[derive(Debug)]
struct Player {
    nick_name: String,
    current_point: i32,
    attendance: i32,
}

impl Player {
    #[allow(dead_code)]
    fn calculate_score(&self, _median: f32) -> f32 {
        // 0.0 // Birinci durum
        ((self.current_point * self.attendance) as f32) * _median
    }
}

/*
    İçinde bilinçli olarak exception fırlattığımız(pardon panic ürettiğimiz) fonksiyonlara ait testlerde,
    "ben zaten böyle bir exception olmasını istiyorum" diyebiliriz. #[should_panic] niteliği bunun için kullanılmaktadır.
    Person struct'ı için yazdığımız new isimli metoda ait test fonksiyonunda bu durum irdelenmektedir.
    age alanının değerinin 13 ile 18 arasında olması istenmektedir. Eğer böyle değilse ortamda panik havası estirilir.
*/
#[derive(Debug)]
struct Person {
    name: String,
    age: i8,
}

impl Person {
    fn new(_name: String, _age: i8) -> Person {
        if _age > 18 || _age < 13 {
            panic!(
                "Bu oyun eğitim 13-18 yaş arası talebeler içindir. Girilen yaş `{}`",
                _age
            );
        } else {
            Person {
                name: _name,
                age: _age,
            }
        }
    }
}

#[cfg(test)] // test modülü olduğunu belirttiğimiz nitelik (attribute)
mod tests {

    use super::*; // bu iç modülden diğerlerine erişebilmek için konuldu. Aksi durumda Player verisine erişemeyiz

    #[test] // test fonksiyonu olduğunu belirttiğimiz nitelik
    fn should_calculated_player_score_positive() {
        let median_value = 0.08;
        let cai = Player {
            nick_name: String::from("cobra kai"),
            current_point: 44,
            attendance: 102,
        };
        let expected_value = cai.calculate_score(median_value);
        assert!(expected_value > 0.0); // assert! makrosu ile kabul kriterimizi yazdık
    }

    #[test]
    fn should_player_nick_name_length_grater_than_three() {
        let gretel = Player {
            nick_name: String::from("han"),
            attendance: 3,
            current_point: 1,
        };
        let result = gretel.nick_name.len() > 3;
        /*
            assert! makrosunu aşağıdaki gibi de kullanabiliriz.
            Bu durumda test sonuçlarına belirttiğimiz metinsel içerik de yansıyacaktır.
            Teste konu olan alanların ve hata sebebinin sonuçlarda görünmesini istediğimiz hallerde işe yarabilir.
        */
        assert!(
            result,
            "Nickname 3 karakterden fazla olmalı. Girilen `{}`",
            gretel.nick_name
        );
    }

    #[test]
    #[should_panic] // Beklediğimiz gibi panik ürettirirsek bu test OK cevabı alır. Aksine test panik ürettirmiyorsa Fail cevabını basar
    fn should_age_available_for_child() {
        let ben_hur = Person::new(String::from("ben hur"), 19);
    }

    /*
        Test fonksiyonlarının, kriterin ihlali sonucu panic oluşturması yerine Err döndürmesi de sağlanabilir.
    */
    #[test]
    fn should_total_greater_than_ten() -> Result<(), String> {
        if 3 + 6 == 10 {
            Ok(())
        } else {
            Err(String::from("Testi geçemedi. Abicim 3+6 10 olur mu?"))
        }
    }

    #[test]
    #[ignore] // ignore niteliği ile bir testi atladığımızı belirtiriz
    fn should_div_work() -> Result<(), String> {
        let x = 10.0;
        let y = 0.0;
        assert_eq!(div(x, y)?, 1.0);
        Ok(())
    }
}

/*
    kobay fonksiyonumuz geriye Ok veya Err döndürmekte.
    should_div_work isimli test fonksiyonunda bu fonksiyonun ? ile kullanıldığına dikkat edelim.
*/
fn div(x: f32, y: f32) -> Result<f32, String> {
    if y == 0.0 {
        Ok(x / y)
    } else {
        Err("Sıfıra bölme hatası".to_owned())
    }
}
