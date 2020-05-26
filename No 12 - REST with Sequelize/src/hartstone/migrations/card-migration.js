module.exports = {

    up: (queryInterface, Sequelize) =>

        queryInterface.createTable('Cards', {
            id: {
                allowNull: false,
                autoIncrement: true,
                primaryKey: true,
                type: Sequelize.INTEGER,
            },
            description: {
                type: Sequelize.STRING,
                allowNull: false,
            },
            attack: {
                type: Sequelize.INTEGER,
                allowNull: false,
                defaultValue: 1,
            },
            health: {
                type: Sequelize.INTEGER,
                allowNull: false,
                defaultValue: 3,
            },
            spell: {
                type: Sequelize.INTEGER,
                allowNull: false,
                defaultValue: 1,
            },
            heroId: {
                type: Sequelize.INTEGER,
                onDelete: 'CASCADE',
                references: {
                    model: 'Heros',
                    key: 'id',
                    as: 'heroId'
                },
            },
            createdAt: {
                allowNull: false,
                type: Sequelize.DATE,
            },
            updatedAt: {
                allowNull: false,
                type: Sequelize.DATE,
            },
        }),

    down: (queryInterface) =>
        queryInterface.dropTable('Cards'),
};