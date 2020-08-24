DROP TABLE Product IF EXISTS;

CREATE TABLE Product (
    product_id BIGINT IDENTITY NOT NULL PRIMARY KEY,
    name VARCHAR(60),
    unit_price DECIMAL(7,2),
    quantity INTEGER
)