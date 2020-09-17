/*
    Kendi geliştirdiğimiz türler veya hash map gibi diğer koleksiyonlar için
    kendi iterator fonksiyonlarımızı da yazabiliriz.
    Tek yapmamız gereken Iterator trait'ini uygulamaktır. Ancak bunun için uygun senaryolara da ihtiyacımız vardır.

    Şunu da bir açıklığa kavuşturalım. Iterator demek veri için bir sonraki veriyi döndüren ve nerde durması gerektiğini bilen bir next fonksiyonu demektir.
*/

// Öğrencinin ders ortalamalarını tutan bir veri yapısı düşünelim
struct Point {
    math: f32,
    lang: f32,
    phys: f32,
    chem: f32,
    vart: f32,
}

/*
    Şimdi de bunu kullanan bir öğrenci veri yapısı tasarlayalım.
    Sanırım amaç anlaşıldı. Bir öğrenicinin notlarını for döngüsü ile dönebilmek istiyorum.
    Bu iterasyon sırasında verinin haricinde verinin durumunu ve hangi konumda olduğumu da bilmem lazım.
    O nedenle position ve data isimli iki alanımız var.

    İlk versiyonda points verisini olduğu gibi tutmuştuk. Lakin verinin referansını tutmamız yeterli.
    Tabii Point referansını tutacağız ama Rust, Student veri yapısının taşıyacağı bu referans ile olan ilişkinin ömrünü bilemeyecek.
    O nedenle <'a> ile lifetime ilişkisini eşitliyoruz.
*/
struct Student<'a> {
    fullname: String,
    school: String,
    position: i32,
    points: &'a Point,
}

/*
    iterator trait'inin uygulanması.
    Eğer <'_> şeklinde isimsiz lifetime bildirimi yapmazsak 'implicit elided lifetime not allowed here' şeklinde hata alırız.
    Bu nedenle <'_> şeklinde bir bildirim yapıp Rust derleyicisinden bu hatayı göz ardı etmesini rica ediyoruz.
*/
impl Iterator for Student<'_> {
    type Item = f32; // Point struct'tındaki türden olduğunda dikkat edelim
                     /*
                         next sıradaki Item'ı yani puanı yani f32 türünden öğeyi döndürür.
                         Kiminkini peki? Self ile ifade ettiğimize göre o anki Student nesnesininkini.
                     */
    fn next(&mut self) -> Option<Self::Item> {
        match self.position {
            0 => {
                self.position += 1;
                Some(self.points.math)
            }
            1 => {
                self.position += 1;
                Some(self.points.lang)
            }
            2 => {
                self.position += 1;
                Some(self.points.phys)
            }
            3 => {
                self.position += 1;
                Some(self.points.chem)
            }
            4 => {
                self.position += 1;
                Some(self.points.vart)
            }
            _ => None,
        }
    }
}

fn main() {
    // ant_man'ın ders not ortalamalarını girdik
    let some_points = Point {
        math: 78.0,
        chem: 55.0,
        phys: 80.0,
        lang: 90.0,
        vart: 67.5,
    };
    let ant_man = Student {
        fullname: String::from("Ant-Man"),
        school: String::from("Mystery Forrest High School"),
        points: &some_points, // referans adresi verdiğimize dikkat edelim
        position: 0, // Aslında bu atama ile iterator'un 0ncı konuma inmesini sağlıyoruz.
    };

    println!("{} ({})", ant_man.fullname, ant_man.school);
    // bu for döngüsü ant_man'ın tüm ders notlarını dolaşabiliyor.
    // Iterator implementasyonu sayesinde
    for p in ant_man {
        println!("{}", p);
    }
}
