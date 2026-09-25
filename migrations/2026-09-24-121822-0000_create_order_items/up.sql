-- Your SQL goes here
CREATE TABLE order_items(
    order_id INT,
    CONSTRAINT fk_orders
    FOREIGN KEY (order_id)
    REFERENCES orders(id),
    quantity INT NOT NULL,
    PRICE INT NOT NULL,
    product_id INT ,
    CONSTRAINT fk_product_id
    FOREIGN KEY (product_id)
    REFERENCES products(id), 
    created_at TimeStamp
)