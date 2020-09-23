/*
    Mountebank ile ilgili yaygın imposter senaryolarından birisi de CSV gibi içeriklerden okunan veriyi döndürmek.
    Örneğin bir veritabanından gelen id değerine göre şehir bilgisi döndüren bir servisimiz olduğunu düşünelim.
    Test senaryomuzda asıl servis yerine onu taklit eden bir servis kullanmak istiyoruz.

    Aşağıdaki gibi bir stub yapısı kullanılabilir.

    Cities/1 gibi bir HTTP talebi olursa,
    fromDataSource kısmında belirtilen CSV dosysını regex ile sorguluyoruz.
    Desenimiz city_id alanını index kabul ederek içeriden bu alana ait satırı buluyor.
    Bulunan satır row değişkenine alınıyor ve body kısmındaki map tekniği ile bir JSON sonuç üretiliyor.
    Test senaryosu böylece gerçekte veritabanına gitmeyen ama ihtiyacımız olan şehir bilgisi döndürecek taklitçi ile akışını devam ettirebilir.
*/
const ports = require('./ports');
const fetch = require('node-fetch');

function register() {
    const stub = [
        {
            predicates: [{
                and: [
                    { equals: { method: "GET" } },
                    { startsWith: { "path": "/cities/" } }
                ]
            }],
            responses: [
                {
                    is: {
                        statusCode: 200,
                        headers: {
                            "Content-Type": "application/json"
                        },
                        body: '{ "cityName": "${row}[name]", "cityCode": "${row}[code]" }'
                    },
                    _behaviors: {
                        lookup: [
                            {
                                "key": {
                                    "from": "path",
                                    "using": { "method": "regex", "selector": "/cities/(.*)$" },
                                    "index": 1
                                },
                                "fromDataSource": {
                                    "csv": {
                                        "path": "src/data/cities.csv",
                                        "keyColumn": "city_id"
                                    }
                                },
                                "into": "${row}"
                            }
                        ]
                    }
                }
            ]
        }
    ];

    const imposter = {
        port: ports.city_service,
        protocol: 'http',
        stubs: stub
    };

    const url = `http://127.0.0.1:${ports.server}/imposters`;

    //console.log(JSON.stringify(imposter));

    return fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(imposter)
    });
}

module.exports = { register };