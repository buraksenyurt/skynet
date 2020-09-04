/*
    aynı klasör seviyesinde libraries.rs isimli dosyası ya da  libraries isimli klasör altındaki mod.rs dosyasını arar.
*/
mod libraries;

fn main(){
    let total=libraries::sum(4,5); // libraries klasöründeki mod.rs dosyasında yer alan sum metodunu çağırdık
    println!("{}",total);
}