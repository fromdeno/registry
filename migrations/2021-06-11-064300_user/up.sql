CREATE TABLE user (
	id VARCHAR NOT NULL PRIMARY KEY,
	username VARCHAR(32) NOT NULL UNIQUE,
	email VARCHAR(64) NOT NULL UNIQUE,
	password VARCHAR(32) NOT NULL,
	token VARCHAR(32) NOT NULL,
	verification_code VARCHAR(32) UNIQUE DEFAULT NULL,
	is_verified BOOLEAN NOT NULL DEFAULT 0,
	is_admin BOOLEAN NOT NULL DEFAULT 0,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);