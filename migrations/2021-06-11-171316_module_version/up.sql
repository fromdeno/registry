CREATE TABLE module_version (
	id VARCHAR(36) PRIMARY KEY,
	version VARCHAR(36) NOT NULL,
	created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);