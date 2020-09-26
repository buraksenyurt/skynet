/*
    Concurrent(Eş Zamanlı) ve Parallel (Parelel) programlama...
    Çok çekirdekli işlemcilerin hayatımıza girmesiyle birlikte önem kazanan başlıca iki konu olarak düşünülebilir.
    Rust dilinin güçlü taraflarından birisi de Concurrency konusunda kendini göstermekte.
    Bu zamana kadar ki örneklerde ownership, type safety, borrowing vs gibi konuları gördük. Bunlar bellek güvenliği (memory safety) ve verimlilik açısından Rust'ı öne çıkarak yanlar.
    Bu özellikler sayesinde Concurrent programlama da daha güvenli ve verimli hale geliyor.
    Nitekim pek çok dilin aksine, Concurrent çözümlerde yaşanacak sorunlar çalışma zamanından değil daha derleme aşamasındayken görülebiliyor.
    Kim üretim ortamında gerçekeleşen bir concurrency hatasını geliştirme veya test ortamında tekrarlamaya çalışıp sorunun tam olarak ne olduğunu anlamaya çalışmak için çaba sarf etmek ister ki ;)
    Dolayısıyla Rust'ın bu sorunlara neden olabilecek sıkıntıları henüz derleme aşamasında söylemesi oldukça önemli.
    Rust'ın bu gücü için Fearless Concurrency terimi kullanlıyor.

    Bu arada;
    Concurrent Programming ile birbirlerinden bağımsız olarak çalışan program parçalarını,
    Parallel Programming ile de aynı anda çalışan program parçalarını kastediyoruz...

    Tabii işin sırrı birçok işletim sistemi ve programlama dilinde olduğu gibi Thread'ler ile çalışmak.
*/

use std::thread; // Thread kütüphanemiz
use std::time::Duration; // Sembolik gecikmeler için

fn main() {
    example_one();

    // Burada da main thread'i içerisinde çalışan bir döngü var
    // Ekrana 10 kere Bar yazacak
    println!("Ana thread başladı...");
    for _i in 1..5 {
        println!("Bar");
        thread::sleep(Duration::from_secs(1)); // ve bu ana thread'de 1er saniye gecikmeli çalışacak
    }
    println!("Ana thread bitti...");
    /*
        Bu ilk örnekte dikkat edilmesi gereken iki nokta var.
        A- example_one içerisinde thread'ler henüz bitmese de, yukarıdaki döngü bittiği için uygulama sonlanacak
        ve diğer thread'ler de ölmüş olacaktır.

        B- Ayrıca main içerisindeki sıra nasıl olursa olsun (ki burada example_one içerisindeki thread'ler önce çalışmak üzere
        yazılmıştır) ilk olarak ana thread içerisindeki kod çalıştırılır. Bu sebepten diğer thread'ler başlamadan önce
        27nci satır mutlaka işletilir ve döngü derhal başlar. (Sanırım Main thread'in öncelikli olduğunu düşünebilirim)
    */
}

fn example_one() {
    // Bir thread açtık
    std::thread::spawn(|| {
        println!("1 başladı...");
        for _i in 1..10 {
            // Ekrana 10 defa Foo yazacak
            println!("Foo");
            thread::sleep(Duration::from_secs(2)); // ve herbir yazma sonrası bu thread 2 saniye bekletilecek
        }
        println!("1 bitti...");
    });

    // Burada da ikinci bir thread açtık
    // Bu kez bir vector'ün elemanları üzerinde işlem yaptığımızı varsayıyoruz
    std::thread::spawn(|| {
        println!("2 başladı...");
        for color in vec!["red", "green", "blue"] {
            println!("{}", color);
            thread::sleep(Duration::from_secs(2)); // ve yine 2 saniyelik bir gecikme
        }
        println!("2 bitti...");
    });
}
