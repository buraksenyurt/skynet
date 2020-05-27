const Hero = require('../models').Hero;
const Card = require('../models').Card;

module.exports = {
    async getAllByHero(req, res) {
        try {
            const hero = await Hero.findOne({
                where: {
                    id: req.params.heroId
                }
            });
            console.log(hero.name);

            if (hero) {
                const cards = await Card.findAll({
                    where: {
                        heroId: req.params.heroId
                    }
                })

                res.status(201).send(cards);
            }
            else {

                res.send(404).send("Hero and it's cards not found")
            }
        }
        catch (e) {
            console.log(e);
            res.status(500).send(e);
        }
    },

    async create(req, res) {
        try {
            const card = await Card.create({
                name: req.body.name,
                description: req.body.description,
                attack: req.body.attack,
                health: req.body.health,
                spell: req.body.spell,
                heroId: req.body.heroId
            });
            res.status(201).send(card);
        }
        catch (e) {
            console.log(e);
            res.status(400).send(e);
        }
    }

    // Update ve delete işlevleri eklenmeli
}