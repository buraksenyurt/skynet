const express = require('express');
const bodyParser = require('body-parser');
const app = express();

app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: false }));

require('./routes')(app);

const PORT = 5555;
app.listen(PORT, () => {
    console.log(`Hartstone Game API servisi ${PORT} üstünden hizmettedir ;)`);
})