const Hero = require('../models').Hero;

module.exports = {
    async getAll(req, res) {
        try {
            const heros = await Hero.find({});
            res.status(201).send(heros);
        }
        catch (e) {
            console.log(e);
            res.status(500).send(e);
        }
    },

    async create(req, res) {
        try {
            const hero = await Hero.create({
                name: req.body.name,
                info: req.body.info
            });
            res.status(201).send(hero);
        }
        catch (e) {
            console.log(e);
            res.status(400).send(e);
        }
    }

    // Update ve delete işlevleri eklenmeli
}