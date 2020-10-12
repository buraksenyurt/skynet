package com.azon.cargotracer;

import com.fasterxml.jackson.databind.ObjectMapper;

import org.springframework.amqp.core.Message;
import org.springframework.amqp.core.MessageListener;
import org.springframework.stereotype.Service;

/*
    MessageListener'dan türeyen bu sınıfın ezdiğimiz(override)
    onMessage metodu üzerinden, RabbitMQ tarafında ilgili kuyruğa atılmış mesajın gövdesini yakalayabiliriz.
*/
@Service
public class EventListener implements MessageListener {

    public void onMessage(Message message) {
        // Message tipinin getBody fonksiyonu ile kuyruk mesajının içeriğini aldık
        String content = new String(message.getBody());
        System.out.println("\n" + content + "\n");

        try {
            /*
                İçeriği JSON formatında göndermiştik.
                Jackson isimli paketten yararlanarak bu içeriği Java tarafındaki PackageInfo nesnemize dönüştürebiliriz.
                Gelen JSON içeriğini Java tarafında nesne olarak ele alabilmek için...
            */
            ObjectMapper objectMapper = new ObjectMapper();
            PackageInfo packageInfo = objectMapper.readValue(content, PackageInfo.class);
            System.out.println(
                    packageInfo.SerialNo + "," + packageInfo.State + "," + packageInfo.Weight + "," + packageInfo.Time);
        } catch (Exception e) {
            System.out.println(e.getMessage());
        }
    }
}