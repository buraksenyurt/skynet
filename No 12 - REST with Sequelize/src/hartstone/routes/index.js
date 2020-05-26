const heroController = require('../controller').hero;
const cardController = require('../controller').card;

module.exports = (app) => {

    app.get('/game/api', (req, res) => {
        res.status(200).send({
            data: "Hartstone Oyun API servisi sürüm 1.0"
        })
    })

    app.get('/game/api/hero', heroController.getAll);
    app.post('/game/api/hero', heroController.create);

    app.get('/game/api/hero/:heroId/cards', cardController.getAllByHero);
    app.post('/game/api/card', cardController.create);
}