/*
    Rust dilini diğerlerinden ayıran önemli özelliklerinden birisi referansların yaşam ömürlerinin olması ve bunun yönetimidir.
*/

fn main() {
    /*
        #1 

        Önce lifetime nerede devreye girer anlamak lazım.
        Aşağıdaki kod parçasını ele alalım. İç içe iki scope var.
        Bu kod derlenmeyecektir.
    */

    {
        // ana scope
        let number; // henüz hiçbir şey atamadığımız bir değişken

        {
            // iç scope
            let stranger_thing = 1;
            number = &stranger_thing; // ve number değişkenine iç scope'daki stranger_thing değişkeninin referansını atadık
        } // sorun şu ki tam bu noktada stranger_thing'in ömrü doldu.

        println!("{}", number); // ve bu nedenle number'ı kullanmak istediğimizde(ki halen ana scope içinde olduğu için kullanılabilir) `stranger_thing` does not live long enough şeklinde derleme zamanı hatası alırız
        // bu derleme hatasının sebebi basittir. number, artık serbest kalmış bir bellek adresini kullanmaya çalışmaktadır

        // Rust derleyicisi yukarıdaki senaryoda kapsamları kontrol ederken Borrow Checker isimli bir tekniğe başvurur
    }
}
