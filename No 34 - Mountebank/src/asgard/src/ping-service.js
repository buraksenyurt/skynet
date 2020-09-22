/*
    Bu bir Hello World mock servisi.
    Mountebank'a register ediliyor.

    Mountebank tarafına register edilen bir mock servis imposter olarak tanımlanır.
    Imposter içerisinde stub tanımlaları yer alır. Birden fazla stub tanımı olabilir.
    Stub'larda ne tür talepler için ne tür cevaplar verileceğinin tanımlandığı yer olarak düşünülebilir.
    
    Örneğin aşağıdaki stub tanımında, JSON formatında bir sözleşme(contract) mevcuttur.
    Predicates ile hangi route ve metod için talep alınacağı ifade edilir.
    Response kısmında da bu talep için nasıl bir cevap dönüleceği. Örnekte HTTP 200 OK durum bilgisi ile birlikte basit bir JSON cevap verilmektedir.
    Yani bu sayede mock servisin talebe karşılık ne döndüreceğini tanımlamış oluruz.

    imposter kısmında ise mock servis ile nasıl bir protokol üstünden iletişim kurulacağı,
    hangi porttan yayın yapacağı ve stub sözleşmesinde nelerin yer alacağın dair bilgilere toplanır.
    Örnekte HTTP protokolünün kullanılacağı ifade edilmektedir.
*/
const fetch = require('node-fetch'); // Mountebank servisine Post işlemini kolaylaştıracak
const ports = require('./ports');

function register() {

    const stub = [
        {
            predicates: [{
                equals: {
                    method: "GET",
                    "path": "/ping"
                }
            }],
            responses: [
                {
                    is: {
                        statusCode: 200,
                        headers: {
                            "Content-Type": "application/json"
                        },
                        body: JSON.stringify({ message: "Pong!" })
                    }
                }
            ]
        }
    ];

    const imposter = {
        port: ports.ping_service,
        protocol: 'http',
        stubs: stub
    };

    /*
        Aşağıdaki kod parçasında Mountebank'ın imposters API'sine HTTP Post ile bir talep gönderme işlemi yer alıyor.
        body parametresine yukarırdaki imposter'ın JSON formatına serileştirilen halini gönderdiğimize dikkat edelim.
        Böylece bu mock servisini Mountebank sunucusuna kayıt etmiş ve kullanıma açmıl olacağız.
    */
    const url = `http://127.0.0.1:${ports.server}/imposters`;

    return fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(imposter)
    });

}

module.exports = { register };