CREATE TABLE transactions (
	id INTEGER PRIMARY KEY,
	timestamp INTEGER NOT NULL,
	account TEXT NOT NULL,
	amount REAL NOT NULL,
	category TEXT,
	description TEXT
);

CREATE TABLE tags (
	id INTEGER PRIMARY KEY,
	transaction_id INTEGER,
	value TEXT,

	FOREIGN KEY(transaction_id) REFERENCES transactions(id)
);

CREATE TABLE notes (
	id INTEGER PRIMARY KEY,
	transaction_id INTEGER,
	value TEXT,

	FOREIGN KEY(transaction_id) REFERENCES transactions(id)
);
