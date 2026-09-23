-- Your SQL goes here
CREATE TABLE orders(
    id INT PRIMARY KEY NOT NULL,
    order_number INT NOT NULL,
    PersonID int,
    product_id int,
    CONSTRAINT fk_product
    FOREIGN KEY (product_id)
    REFERENCES products(id),
    delivery_address VARCHAR(255) NOT NULL,
    payment ENUM ('PENDING','DONE') NOT NULL,
    created_at TimeStamp,
    updated_at TimeStamp
)