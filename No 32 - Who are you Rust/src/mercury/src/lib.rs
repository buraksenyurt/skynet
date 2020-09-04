/*
    Örnek bir modül.
    kendi içinde entity, flight_opt ve reports isimli modüller içeriyor.


    mercury library içerisinde yer alan src/lib.rs aynı zamanda kök sandık (Crate root) olarak da adlandırılır.
    Yani crate anahtar kelimesi ile root'a erişip :: operatörü ile iç elementlere inebiliriz.


    flight_opt modülünde visitor_manager modülünde tanımlı Visitor struct'ını kullanmak için nasıl yollar izlediğimize dikkat edelim.
    ilki absolute path formatıdır. Crate ile başlar.


    Absolute path metodunda crate ile bulunduğumuz sandığı işaret ediyoruz.
    :: sonrası bu sandık içerisinde visitor_manager modülüne giriyoruz. sonrasında yine :: ile Visitor isimli struct veri tipine ulaşıyoruz.


    entity modülü içinde kullanılan pub anahtar kelimelerine dikkat edelim. Normalde Visitor isimli struct ve alanları private'tır.
    Yani flight_opt içeriisnde erişilemez.
    Bu nedenle pub ile public hale getirilmişlerdir.

    Bu arada flight_opt modülündeki save_visitor metodu içinden entity modülündeki Visitor struct'ına erişmek için
    super::entity:Visitor notasyonu da kullanılabilir. super aslında dosya sistemini düşünürsek ..'yı ifade eder. Yani bir üst klasör.

*/
mod visitor_manager {
    pub mod entity {
        pub struct Visitor {
            pub fullname: String,
            pub ticket_no: String,
        }

        pub struct Spaceship {
            pub name: String,
            pub flight_no: i32,
            pub passenger_capacity: i8,
        }

        pub struct SpaceLocation(i32, i32, i32);
    }

    mod flight_opt {

        /* 
            use ile flight_opt içerisinde kullanmak istediğimiz modül elemanlarını bir kere tanımlayıp
            yola devam edebiliriz. Yani SpaceLOcation kullanmak istediğimiz her yerde
            Absoulte path veya relative path ya da super kullanarak uzun formatta bildirim yapmak zorunda değiliz.
        */
        use crate::visitor_manager::entity::SpaceLocation;

        fn save_visitor(name: String, ticket: String) {
            let v = super::entity::Visitor {
                fullname: name,
                ticket_no: ticket,
            };

            // let v = crate::visitor_manager::entity::Visitor {
            //     //absoulute path tekniği
            //     fullname: name,
            //     ticket_no: ticket,
            // };
            println!(
                "{} isimli ziyaretçi için merkür yolculuk kaydı açıldı. Bilet numarası {}",
                v.fullname, v.ticket_no
            )
        }

        fn send_spaceship(name: String, no: i32, capacity: i8, target: SpaceLocation) {}
    }

    mod reports {
        fn get_total_visitor(region: String) -> i32 {
            // Merkürdeki üs bazında yolcu sayısını döndürüyor. Mesela :)
            return 1000;
        }
    }
}
