// mountebank ve kendi yazdığımız ports modülünü ekledik
const mb = require('mountebank');
const ports = require('./ports');

// Yeni bir mountebank örneği oluşturuyoruz
const server_instance = mb.create({
    port: ports.server,
    pidfile: '../mb.pid',
    logfile: '../mb.log',
    protofile: '../protofile.json',
    ipWhitelist: ['*']
});