var express = require('express');
var morgan = require('morgan');

var app = express();
app.use(morgan('combined')); // HTTP Loglama middleware bildirimi

// Tipik bir HTTP Get talebi karşılıyoruz
// quotes olarak gelen yönlendirmelerde çalışacak
app.get('/quotes', function (req, res) {
    res.send(quotes);
})

// Varsayılan HTTP Get Talepleri
app.get('/', function (req, res) {
    res.send('pong!');
})

// listen çağrısı ile servis uygulaması ayağa kalkıyor
// Birden fazla port seçeneği olabilir. 
// Process Manager olan PM2 aracı argv[2] üstünden gelen port ile uygulamayı ayağa kaldırabilir
app.listen(process.argv[2] || process.env.PORT || 4500, () => {
    console.log(`Uygulama ayakta ve ${process.argv[2] || process.env.PORT || 4500} nolu porttan dinlemede.`);
});

// Sembolik bir JSON içeriği
// Birkaç özlü söz yer alıyor
var quotes = [
    { "id": 1, "owner": "Nelson Mandela", "content": "The greatest glory in living lies not in never falling, but in rising every time we fall." },
    { "id": 2, "owner": "John Lennon", "content": "Life is what happens when you're busy making other plans." },
    { "id": 3, "owner": "Aristotle", "content": "It is during our darkest moments that we must focus to see the light." },
    { "id": 4, "owner": "Marilyn Monroe", "content": "Keep smiling, because life is a beautiful thing and there's so much to smile about." },
    { "id": 5, "owner": "Oprah Winfrey", "content": "You know you are on the road to success if you would do your job and not be paid for it." },
];