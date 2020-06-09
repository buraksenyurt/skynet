var express = require('express');
var bodyParser = require('body-parser');
const mqtt = require('mqtt');
var app = express();
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

// Önce bir bağlanalım. Docker Container'ın olduğu adrese doğru.
var qt = mqtt.connect("http://localhost:1883");

// MQTT Client paketi olay bazlı çalışır. Kullanımı kolaydır.

// Broker ile bağlantı sağlandığında
qt.on('connect', () => {
    console.log('Eclipse Mosquitto ile bağlantı sağlandı');
});

// Bir hata oluştuğunda
qt.on('error', (err) => {
    console.log(`Bir hata oluştu sanırım. ${err}`);
    qt.end();
});

// İstemci HTTP Post üstünden Broker'a iletilmek üzere bir istek aldığında
app.post("/input", function (req, res) {
    // HTTP Body ile gelen JSON içeriği alıyoruz
    var payload = req.body;
    // Bu içerikteki gate niteliğinin değerini topic olarak
    // identity niteliğini değerini de mesaj olarak kullanıyoruz
    // ve Broker'a gönderiyoruz
    qt.publish(payload.gate, payload.identity);
    console.log(`Broker'a ${payload.gate} konusunda ${payload.identity} mesajı gönderildi`);
    res.status(200).send('Mesaj Brokera gönderildi'); //İstemciye de HTTP 200 Ok gönderelim
});


app.listen(4444, function () {
    console.log("Uygulama 4444 nolu porttan hizmette");
});