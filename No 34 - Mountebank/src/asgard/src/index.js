// mountebank ve kendi yazdığımız ports modülünü ekledik
const mb = require('mountebank');
const ports = require('./ports');

/* 
    Mountebank uygulamasını ayağa kaldırdığımızda, yazdığımız mock servislerin de 
    etkinleştirilmesini sağlayabiliriz.
    then fonksiyonuna odaklanın.
*/
const pingService = require('./ping-service');

// Yeni bir mountebank örneği oluşturuyoruz
const server_instance = mb.create({
    port: ports.server,
    pidfile: '../mb.pid',
    logfile: '../mb.log',
    protofile: '../protofile.json',
    ipWhitelist: ['*']
}).then(function () {
    pingService.register();
});