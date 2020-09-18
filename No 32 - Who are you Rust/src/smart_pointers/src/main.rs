/*
    Pointer denilince aklımıza bellekteki bir bölgenin adresini tutan işaretçi gelir.
    Rust dilinde pointer'ların en bilinen tipi referans türleridir. 
    Ancak birde Smart Pointer adı verilen veri yapısı vardır. Smart Pointer'da verinin adresini taşır ama 
    ekstradan metadata bilgisi de barındırır.
    Hatta şu ana kadar birkaç smart pointer kullanmışızdır. String ve Vec veri yapıları Smart Pointer türündendir.
    Sadece rerenas adresi taşımazlar bununla birlikte ek bilgi de barındırırlar. Vector'ün başlangıç kapasitesi gibi.

    Bu arada pointer olarak belirtilen referans türleri veriyi ödünç alırlar(borrowing durumu)
    Aksina Smart Pointer'lar adreslenen veriyi sahiplenirler (ownership durumu)

    Bu örnekte smart pointer türlerini incelemeye çalışacaım.
*/
fn main() {
    /*
        En bilinen Smart Pointer türlerinden birisi Box<T>
        Veriyi Heap üzerinde tutmamızı sağlar. Tamam referans türü ile de bunu yapıyoruz ama metadata olayını unutmayalım.
        Ayrıca,
            Büyük bir verinin sahiplini kopyalamadan taşımak istediğimizde (Büyük veriyi Heap'te kutulayacağız)
            Derleme zamanında boyutunu bilemediğimiz bir veri kullandığımızda
        gibi durumlarda tercih edilir.
    */

}
