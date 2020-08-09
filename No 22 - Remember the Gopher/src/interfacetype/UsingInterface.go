package main

/*
	Go dilinde Interface tipi hangi amaçla kullanılır buna bakıyoruz.
	Interface tipi ile içinde metotlar olan bir sözleşme tanımlanır.
	Bu sözleşmedeki metotları uygulayan diğer tipler, ilgili interface tipince de taşınabilir.
*/

import (
	"fmt"
)

func main() {
	alfonso := Player{"Alfonso De La Puante"}
	var handler EventHandler

	// Önemli olan kısım burası. alfonso bir Player değişkeni.
	// handler isimli EventHandler türünden değişkene atanabiliyor.
	// Nitekim Player tipi EventHandler üzerinde belirtilen Loaded ve Played isimli metotları uyguladı.

	handler = alfonso
	fmt.Println(handler.Loaded())
	fmt.Println(handler.Played())

	jessica := Player{"Jessica La Maria Qruze La Fiesta El Matador"}
	// Aşağıdaki atamadan sonra handler değişkeni artık jessica için çalışacak.
	handler = jessica
	fmt.Println(handler.Loaded())
	fmt.Println(handler.Played())
	fmt.Println(handler.Played())
}

//EventHandler isimli interface Loaded isimli bir metot tanımlanmıştır
type EventHandler interface {
	Loaded() string
	Played() string
}

//Player isimli kobay bir struct tipi
type Player struct {
	name string
}

//Played metodu. Bu metodun Player tipine uygulandığına dikkat edelim.
//Yani EventHandler isimli interface içinde tanımlanmış olan metot.
func (p Player) Played() string {
	return fmt.Sprintf(p.name + " az önce oynadı.")
}

//Loaded isimli metot tanımı da EventHandler interface tipi için yer almaktadır.
func (p Player) Loaded() string {
	return fmt.Sprintf(p.name + "  profili sisteme yüklendi.")
}
