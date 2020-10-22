const express = require('express')
const app = express()

app.get('/ping', function (request, result) {
    result.send('Biraz ara verip Ping Pong oynayalım mı?')
})

app.listen(5555, "0.0.0.0", function () {
    console.log('Servisimiz http://localhost:5555/ping adresinden denenebilir.')
})