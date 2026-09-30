-- Your SQL goes here
ALTER TABLE orders
ADD CONSTRAINT fk_users
FOREIGN KEY (user_id)
REFERENCES users(id);