/*
    Pek çok programlama dilinde enum tipi mevcut.
    Sayısal olarak ifade edilen sabitleri isimlendirerek kullandığımız tipler olarak düşünebiliriz.
    Rust dilinde de enum desteği var ama bazen struct'lar yerine de tercih edilebiliyorlar. Öyle ki enum içindeki değişkenler başka veri türlerini ele alarak kullanılabiliyorlar. Enteresan değil mi?
*/

// Önce örnek bir enum nasıl tanımlanıyor bakalım
enum TaskSize {
    Small,
    Medium,
    Large,
    Xlarge,
}

// Şimdi de yukarıdaki enum sabitini de kullanan bir struct tanımladık
struct Task {
    size: TaskSize,
    title: String,
}

// Lakin yukarıdaki gibi bir kullanım yerine struct verisini içeren bir enum tipi de tanımlanabiliyor
enum Job {
    Small(String, i32), // Parantez içerisindeki String kısımları Task struct'ı içerisindeki title yerine geçiyor. i32 ile de işin büyüklüğünü ifade edebiliriz
    Medium(String),
    Large(String),
    Xlarge(String),
}

// Hatta enum veri yapısındaki değişkenler primitive türler gibi bir struct'ı da kullanabilirler
struct Detail {
    title: String,
    business_value: i32,
}
enum Action {
    Small(Detail), //Action değişkenleri Detail isimli struct veri yapısını içerir
    Medium(Detail),
    Large(Detail),
}

// Enum veri yapısı her değişkeni farklı sayıda ve türle çalışacak şekilde de tanımlanabilir.
enum Status {
    Done,                                       // Bir veri ile ilişkili değil. Standart enum sabiti.
    Error { reason: String, impact_size: i8 }, // Error değişkeni anonymous bir struct içerir
    Log(String),                                // Log değişkeni ise bir String içerecektir
}
// Yukarıdaki Status isimli veri yapısı struct'lar ile aşağıdaki şekilde de ifade edilebilirdi.
struct StatusDone;
struct StatusError {
    reason: String,
    impact_size: i8,
}
struct StatusLog(String); //Tuple Struct

fn main() {
    // Enum içindeki bir değişken aşağıdaki gibi atanabilir
    let small = TaskSize::Small;

    // Bir görevi büyüklüğü ile tanımladığımız struct değişkeninin örnek tanımı
    let install_git = Task {
        size: TaskSize::Medium,
        title: String::from("Ubuntu ortamına git kurulacak"),
    };
    // Job enum tipinden bir değişkeni de aşağıdaki gibi oluşturabiliriz
    let install_docker = Job::Small(
        String::from("Heimdall üstünde Docker kurulumu yapılmalı."),
        5,
    );

    // Action veri yapısı(ki enum tipidir) değişklenleri Task isimli struct'ı kullanıyor.
    let micro_service = Action::Large(Detail {
        title: String::from("Müşteri modülünün mikro servise dönüşümü."),
        business_value: 13,
    });
}
