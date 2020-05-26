module.exports = (sequelize, DataTypes) => {

    let Hero = sequelize.define('Hero', {
        name: DataTypes.STRING,
        info: DataTypes.STRING
    });

    Hero.associate = function (models) {
        Hero.hasMany(models.Card, {
            foreignKey: 'cardId',
            as: 'cards'
        });
    };

    return Hero;
}