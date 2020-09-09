/*
    Rust dilini diğerlerinden ayıran önemli özelliklerinden birisi referansların yaşam ömürlerinin olması ve bunun yönetimidir.
    Genelde lifetime ile scope kavramları birbirine karıştırılır. Lakin aynı şey değillerdir.
    Bir değişkenin hayatı oluşturulduğunda başlar ve yok edilene kadar(destroy) devam eder.
*/

fn main() {
    /*
        lifetime noktalarını daha iyi anlamak için şu kod parçasına bakalım.
        x ve y, en fazla yaşam ömrü olan number değişkeninin referansını kendi yaşam süreleri boyunca ödünç alıp kullanıyorlar.
    */

    let number = 3.14; //------------------------------------> number lifetime start

    {
        let x=&number; //--------------------> x lifetime start
        println!("{}",x);
    }//--------------------------------------> x lifetime end

    {
        let y=&number; //--------------------> y lifetime start
        println!("{}",y);
    }//--------------------------------------> y lifetime end
} //----------------------------------------------------------> number lifetime end
