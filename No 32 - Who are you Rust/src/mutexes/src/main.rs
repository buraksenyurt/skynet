/*
    Mutex...

    Rust dilinde kanallardan yararlanarak mesajlaşma yapan eş zamanlı thread'leri nasıl kullanabileceğimizi gördük.
    Kanallar (Channels) aslında tekil mülkiyet hakkı sağlarlar. Yani bir thread kanala bir veri bıraktığında bu veri onun için artık kullanılabilir değildir.
    Buna karşın birden fazla thread'in aynı anda aynı bellek bölgesini kullanmak isteyeceği durumlar da mümkündür.
    Bu tip durumların kanallar haricinde bir diğer yönetim şekli de Mutex tipini ele almaktır.
    Kanallar nasıl tekip mülkiyetliği baz alıyorsa, Mutex tipi de smart pointer'lar gibi çoklu mülkiyet/sahiplik hakkını baz alırlar(Keza Mutex<T> bir smart pointer'dır)
    Özetle çoklu thread'lerin aynı veriyi kullanmak istemeleri halinde t anında sadece bir tanesinin onu kullanmasına nasıl izin vereceğimizi görmek istiyorum.
    Mutex bu anlamda bakalım nasıl bir çözüm sunuyor görelim.

    Bir thread Mutex'e alınan bir veriyi kullanmak istediğinde bunu öncelikle ona sorar ve eğer müsaitse verinin kilidini(lock) alır. İşi bitince de
*/
use std::sync::Mutex;
use std::thread;

fn main() {
    /*
        #1

        İki thread Mutex kullanırken.
        worker_sue içinde "use of moved value" derleme zamanı hatası alırız.
        *collector satırında ise "borrow of moved value" derleme zamanı hatasını alırız.
        
    */
    // Mutex nesnesini oluşturuyoruz. Tutacağı veri 32 bit integer ve değeri 1
    let collector = Mutex::new(1);

    // Bir thread başlatılıyor
    let worker_joe = thread::spawn(move || {
        // mutex'in kilidini alıyoruz ve işimiz bitene kadar thread'i blokluyoruz
        let mut point = collector.lock().unwrap();
        // Burada kilidi bizde olan Mutex verisinin değerini değiştirdik
        *point += 5;
    });
    // Scope dışına çıktığımız anda Mutex'in kilidini de devretmiş olduk. Dolayısıyka başka bir thread artık bu kilidi alabilir.

    let worker_sue = thread::spawn(move || {
        let mut point = collector.lock().unwrap();
        *point += 3;
    });

    worker_joe.join().unwrap();
    worker_sue.join().unwrap();

    println!("{}", *collector.lock().unwrap());
}
