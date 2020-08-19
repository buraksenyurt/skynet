package com.skynet.gamesworldapi;

import java.util.List; // Metodumuz bir liste döndüreceği için eklenen paket

import org.springframework.data.mongodb.repository.MongoRepository;
import org.springframework.data.repository.query.Param;
import org.springframework.data.rest.core.annotation.RepositoryRestResource;

/*
    MongoRepository türünü genişleten bir interface söz konusu.
    İçinde seviyeye göre oyuncu listesi döndüren ek bir fonksiyonellik tanımı da var.
    path ile bu repository için API EndPoint'ini tanımlamış olduk
*/

@RepositoryRestResource(collectionResourceRel = "player", path = "player")
public interface PlayerRepository extends MongoRepository<Player, String> {
    /*
        Geriye Player türünden bir liste döndürecek.
        Amaç isActive değerine göre etkin olan veya olmayan oyuncu listesini çekmek
        _isActive.
        Bu arada metot adının getBy_isActive olması tesadüf değil. Player sınıfındakinda isActive field'ını bu şekilde isimlendirdiğimiz için.
        Aksi durumda build sırasında hata alırız. Kod derlenmez. Test çıktısı da fail olur.
    */
    List<Player> getBy_isActive(@Param("active") Boolean isActive);
}