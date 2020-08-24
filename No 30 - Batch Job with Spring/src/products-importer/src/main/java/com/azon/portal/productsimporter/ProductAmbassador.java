package com.azon.portal.productsimporter;

/*
    Intermediate katmanında devreye giren bu ara sınıf aslında verinin ele alınıp dönüştürme işlemlerinin yapıldığı yerdir.
    Sınıf dikkat edileceği üzere ItemProcessor arayüzünden türetilmektedir.
    Örnekte Product tipinden bir Input gelip yine aynı tipten Output nesnesi döndürülmektedir
*/
import org.springframework.batch.item.ItemProcessor;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ProductAmbassador implements ItemProcessor<Product, Product> {

    // Loglamayı nasıl yapıyoruz anlamak için ekledim.
    private static final Logger logger = LoggerFactory.getLogger(ProductAmbassador.class);

    @Override
    public Product process(final Product product) throws Exception {
        /*
         * Normalde burada gelen Product nesnesi üzerinde bir takım işlemler
         * yapılabilir. Mesela birim fiyatlar sistemin istediği para birimine o anki
         * kurdan çevrilebilir vb
         */
        logger.info(product.toString() + " Şeklinde içerik geldi");
        return product;
    }
}