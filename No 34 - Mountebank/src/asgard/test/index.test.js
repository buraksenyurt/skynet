const expect = require('chai').expect;
const axios = require('axios');

describe('Asgard Mock Servis testleri', () => {

    it('Herhangi bir şehirden en az bir kullanıcı bilgisi gelmeli', () => {
        var city;

        return axios
            .get(`http://localhost:5502/cities/3`)
            .then(res => res.data)
            .catch(error => console.log(error))
            .then(response => {
                /*
                    Beklentilerimizi yazıyoruz.
                    Mock servisinin dönüşü bir object olmalı,
                    cityName özelliği bulunmalı ve değeri Istanbul olmalı
                    ayrıca cityCode özelliğinin değeri de 340 gelmeli
                */
                expect(typeof response).to.equal('object');
                expect(response.cityName).to.equal('Istanbul')
                expect(response.cityCode).to.equal("340")

                city = response.cityName; // sonraki işlemler için değişkeni sakladım sadece
            }).then(() => {
                // Burada başka bir test operasyonu icra edilebilir
                // console.log(city, "ile ilgili başka testler");
            });
    });

    it('Ping mesajıma karşılık Pong denmeli ve oyun başlamalı', () => {
        return axios
            .get(`http://localhost:5501/ping`) // servis adresini bozup Fail durumunu da test edebiliriz
            .then(res => res.data)
            .catch(error => console.log(error))
            .then(response => {
                expect(typeof response).to.equal('object');
                expect(response.message).to.equal('Pong!')
            });
    });
});