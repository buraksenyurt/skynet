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
            nick_name: String::from("ha"),
            attendance: 3,
            current_point: 1,
        };
        assert!(gretel.nick_name.len() > 3);
    }
}
