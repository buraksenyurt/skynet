module.exports = (sequelize, DataTypes) => {

    let Card = sequelize.define('Card', {
        name: DataTypes.STRING,
        description: DataTypes.STRING,
        attack: DataTypes.INTEGER,
        health: DataTypes.INTEGER,
        spell: DataTypes.INTEGER
    });

    Card.associate = function (models) {
        Card.belongsTo(models.Hero, {
            onDelete: "CASCADE",
            foreignKey: 'heroId'
        });
    };

    return Card;
}