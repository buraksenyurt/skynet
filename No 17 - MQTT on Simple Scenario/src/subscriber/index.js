const mqtt = require('mqtt');
var express = require('express');
var app = express();

// Hemen bağlantımızı sağlayalım
var qt = mqtt.connect("http://localhost:1883");

// westSide isimli topic için abonelik başlatıyoruz
qt.subscribe('west side', { qos: 0 });

// Broker ile bağlantı sağlandığında
qt.on('connect', () => {
    console.log('Eclipse Mosquitto ile bağlantı sağlandı');
});

// Broker ile olan bağlantı kapatıldığında
qt.on('close', () => {
    console.log('Mosquitto ile bağlantı kesildi');
});

// Broker'a belli bir konuda bir mesaj geldiğinde
qt.on('message', function (topic, message) {
    console.log(`${topic} konusunda ${message} şeklinde bir mesaj geldi.`);

});

// Bir hata oluştuğunda
qt.on('error', (err) => {
    console.log(`Bir hata oluştu sanırım. ${err}`);
    qt.end();
});

app.listen(4445, function () {
    console.log("Uygulama 4445 nolu porttan ayakta ve dinlemede");
});