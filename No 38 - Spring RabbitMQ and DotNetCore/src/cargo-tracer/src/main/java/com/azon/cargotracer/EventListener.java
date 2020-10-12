package com.azon.cargotracer;

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
        System.out.println("\n" + new String(message.getBody()) + "\n");
    }
}