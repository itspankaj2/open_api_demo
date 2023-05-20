-- Your SQL goes here
CREATE TABLE users (
  userId SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  dob INT,
  location VARCHAR(255)
);
