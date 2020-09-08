/*
    C# tarafında uzun zamandır generic mimariye aşinayım. Bakalım Rust tarafında generic ile kastedilen şey ne?

    Resmi dokümana göre generic türler özellikle kod tekrarını önlemek için tercih ediliyor.
    sum_of_two isimli fonksiyonu ele alarak konuyu irdeleyelim.
    fonksiyon i16 tipinden iki sayıyı alıp toplamını geriye döndürüyor.
*/

use std::ops::Add;

fn sum_of_two(x: i16, y: i16) -> i16 {
    return x + y;
}

fn main() {
    // #1
    let r1 = sum_of_two(1, 6);
    println!("{}", r1);
    /*
        Şimdi bu fonksiyonu aşağıdaki gibi çağırmayı denersek i16 türünden parametre beklediğine dair derleme zamanı hatası ile karşılaşırız.
        Çözüm olarak sum_of_two'nun f32 türü ile çalışacak bir versiyonunu yazabiliriz ama bu kod tekrarnın en canlı örneği olur.
        Bunun yerine generic bir fonksiyon da geliştirebiliriz (yani sum fonksiyonu)
    */
    //let r2 = sum_of_two(1.2, 6.4);
    let r3 = sum(19, 4);
    let r4 = sum(3.14, 2.56);
    println!("{}\n{}", r3, r4);

    // Generic strcut kullanımı örneği
    let cmp1 = Complex { r: 18, v: 1.56 };
    println!("{}+{}i", cmp1.r, cmp1.v);
    let cmp1 = cmp1.change(); // let ile yapılan atamayı kaldırdığınızda aşağıdaki satır için bir hata alacaksınız. Sizce sebebi ne olabilir?
    println!("{}+{}i", cmp1.r, cmp1.v);
}

/*
    Generic fonksiyon örneği.
    sum fonksiyonu T türünden parametreler ile çalışıp yine T türünden sonuç döndürecek şekilde yazıldı.
    Ancak dikkat edilmesi gereken bir nokta var.

    T'nin tanımlanmasında Add şeklinde başka bir ifade daha yer almaktadır. Buradaki Add bir Trait'tir.
    T tipinin sahip olması gereken bir davranışı(iki T nin toplanabilmesi özelliğini) belirtiyoruz.
    Eğer Add Trait'ini kullanmazsak T'nin T'ye eklenemeyeceğine dair bir hata mesajı alırız.
    
    Trait'leri traits isimli örnekte ele alıyoruz.
*/
fn sum<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

/*
    #2
    Pek tabii bir struct içinde de ve hatta struct'a ait metotlarda da generic yaklaşımı kullanılabilir.
    Aşağıdaki Complex isimli struct'ın alanları T ve U türündendir. Ne atarsak o.

    Complex sınıfına entegre edilen change metodu kompleks sayının gerçel ve sanal köklerinin yerini değiştirip yeni bir Complex türünü geriye döndürmektedir
*/
struct Complex<T, K> {
    r: T,
    v: K,
}

impl<T, K> Complex<T, K> {
    fn change(self) -> Complex<K, T> {
        Complex {
            r: self.v,
            v: self.r,
        }
    }
}
