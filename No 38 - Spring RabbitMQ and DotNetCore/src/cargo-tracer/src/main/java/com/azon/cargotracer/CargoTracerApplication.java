package com.azon.cargotracer;

import java.io.IOException;

import org.springframework.amqp.rabbit.connection.ConnectionFactory;
import org.springframework.amqp.rabbit.listener.SimpleMessageListenerContainer;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@SpringBootApplication
@Configuration // RabbitMQ sunucu adı, bağlantı portu, kullanıcı adı vb bilgileri application.properties'ten almasını söyledik
public class CargoTracerApplication {

	@Bean
	SimpleMessageListenerContainer container(ConnectionFactory connectionFactory) {
		SimpleMessageListenerContainer container = new SimpleMessageListenerContainer();
		container.setConnectionFactory(connectionFactory);
		container.setQueueNames("package-state-action"); // Dinlemek istediğimiz kuyruğu belirttik
		container.setMessageListener(new EventListener()); // Kanalı dinleyen ve mesaj geldiğinde bunu ekrana basacak
															// servis sınıfını belirttik
		return container;
	}

	public static void main(String[] args) throws InterruptedException, IOException {
		SpringApplication.run(CargoTracerApplication.class, args);
	}
}
