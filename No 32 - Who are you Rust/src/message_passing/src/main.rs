/*
    thread'lerin ortak veriler üzerinde işlem yapması gerektiği durumlarda
    eşzamanlılığı güvenli bir şekilde sağlamak için mesajlaşma tekniği uygulanır.

    Go'dan gelen motto burada da geçerliliğini korur;
    "Hafızayı paylaşarak iletişim kurmayın; bunun yerine iletişim kurarak hafızayı paylaşın"

    Rust dilinde de Go'dakine benzer channel kullanımı söz konusu. Kanaldan faydalanarak
    thread'ler aralarında haberleşebiliyorlar.

    Bir channel, verici(Transmitter) ve alıcı(Receiver) olmak üzere iki parçadan oluşuyor.

    Örneğin n adet thread'in bir hesaplama yapıp bu hesaplamaları işlenmek(aggregate) üzere başka bir
    thread'e gönderdiğini düşünelim. Bu channel kullanımı için ideal bir senaryodur. Keza
    chat uygulamaları için de channel kullanımı söz konusudur.
*/
use std::sync::mpsc; // Multiple Producer Single Consumer
use std::thread;
use std::time::Duration;

fn main() {
    /*
        #1

        İlk olarak bir kanal nasıl açılır, bu kanale bir thread üstünden mesaj nasıl basılır ve
        tabii basılan mesaj başka bir thread tarafından nasıl alınır bakalım.

        channel tanımlandığında geriye bir tuple döner
        tx, transmitter(yayıncı) rx ise receiver(alıcı) nesneleri işaret eder
    */
    let (tx, rx) = mpsc::channel();

    // move kullandık ki tx'i closue ile kullanabilelim
    let worker1 = thread::spawn(move || {
        println!("#1 Jennifer tepeden nehre bir plastik ördek bırakıyor. Aklında bir sayı var.");
        let calculated_value = 3.1415;
        tx.send(calculated_value).unwrap(); // transmitter ile kanala mesajımızı/değeri bırakıyoruz
        
        /*
            worker'lar pek tabi eş zamanlı olarak işe başlarlar.
            Aşağıda worker1 sembolik olarak uzun süre bir iş yapsa da,
            yukarıda kanala bir mesaj bırakmıştır ve diğer thread'ler 
            bu mesajı duraksatmaya aldırmadan alıp kullanabilirler ;)
        */
        thread::sleep(Duration::from_secs(3)); 
    });

    let worker2 = thread::spawn(move || {
        println!("#2 Alice, Jennifer'ın gönderdiği plastik ördeği bekliyor.");
        let received_value = rx.recv().unwrap(); //receiver ile kanala bırakılan mesajı yakalıyoruz
        println!("Kanala bırakılan plastik ördeğin aklındaki sayı: {}",received_value);
    });

    worker1.join().unwrap();
    worker2.join().unwrap();
}
