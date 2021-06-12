CREATE TABLE module (
	id VARCHAR(36) NOT NULL PRIMARY KEY,
	token VARCHAR(36) NOT NULL UNIQUE,
	name VARCHAR(36) NOT NULL,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	owner VARCHAR(36) NOT NULL,
	FOREIGN KEY (owner) REFERENCES user(id)
	/* TODO research about on delete cascade */
);