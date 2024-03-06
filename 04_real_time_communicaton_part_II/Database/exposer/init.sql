-- Tables
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    user_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(100) NOT NULL 
);

CREATE TABLE  IF NOT EXISTS accounts (
    id SERIAL PRIMARY KEY,
    account_number INT UNIQUE NOT NULL,
    account_status INT NOT NULL,
    user_id int NOT NULL,
    CONSTRAINT fk_user_id 
        FOREIGN KEY (user_id) 
            REFERENCES users(id)
);

CREATE TABLE  IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    account_id int NOT NULL,
    amount int NOT NULL,
    CONSTRAINT fk_account_id
        FOREIGN KEY (account_id)
            REFERENCES accounts(id)   
);


-- Test data function
CREATE OR REPLACE FUNCTION generate_test_data(num_users INT, num_accounts_per_user INT, num_transactions_per_account INT)
RETURNS void AS $$
DECLARE
    user_id INT;
    account_id INT;
    i INT;
    j INT;
    k INT;
BEGIN
    FOR i IN 1..num_users LOOP
        INSERT INTO users (user_name, email, password)
        VALUES ('User'||i, 'user'||i||'@example.com', 'password'||i)
        RETURNING id INTO user_id;

        FOR j IN 1..num_accounts_per_user LOOP
            INSERT INTO accounts (account_number, account_status, user_id)
            VALUES (floor(random()*1000000)::INT, floor(random()*10)::INT, user_id)
            RETURNING id INTO account_id;

            FOR k IN 1..num_transactions_per_account LOOP
                INSERT INTO transactions (account_id, amount)
                VALUES (account_id, floor(random()*10000)::INT);
            END LOOP;
        END LOOP;
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- View
CREATE OR REPLACE VIEW tranction_view AS
SELECT u.user_name, a.account_number, t.amount 
FROM users u
INNER JOIN accounts a ON a.user_id = u.id
INNER JOIN transactions t ON t.account_id = a.id;


-- USERS AND PRIVILEGES

CREATE USER myreaduserall PASSWORD '123';
GRANT SELECT ON ALL TABLES IN SCHEMA public TO myreaduserall;

CREATE USER myreadusersome password '123';
GRANT SELECT ON accounts, transactions TO myreadusersome;

CREATE USER mywriteuserall password '123';
GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA public TO mywriteuserall;

CREATE USER mywriteusersome password '123';
GRANT SELECT,INSERT,UPDATE,DELETE ON accounts TO mywriteusersome;

CREATE USER myviewuser password '123';
GRANT SELECT ON tranction_view TO myviewuser;

CREATE USER myfunctionuser password '123';
GRANT EXECUTE ON FUNCTION generate_test_data(INT, INT, INT) TO myfunctionuser;

CREATE USER mycolumnuserread password '123';
GRANT SELECT(id, user_name) ON users TO mycolumnuserread;
