const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');

const config = {
    port: 3000,
    host: '0.0.0.0',
};

const app = express();

app.use(bodyParser.json());
app.use(cors());

app.get('/lucky', (req, res) => {
    res.status(200).send('Sturdy: (of a person or their body) strongly and solidly built');
});

app.listen(config.port, config.host, (e)=> {
    if(e) {
        throw new Error('Internal Server Error');
    }
    console.log('Word Api is running and listening on 0.0.0.0:3000.')
});