/// SQL query to create the USER table for SQLite.
pub const CREATE_TABLE_USER: &str = r#"
CREATE TABLE USER (
 	id INTEGER PRIMARY KEY, -- implies auto-increment in SQLite
 	username TEXT NOT NULL UNIQUE,
 	password TEXT NOT NULL,
 	created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- SQLite uses TEXT for TIMESTAMP and DATETIME
 	updated_at TEXT DEFAULT CURRENT_TIMESTAMP -- ON UPDATE CURRENT_TIMESTAMP is not directly supported by SQLite
);
"#;

/// SQL query to create the ACCOUNT table for SQLite.
pub const CREATE_TABLE_ACCOUNT: &str = r#"
CREATE TABLE ACCOUNT (
    id INTEGER PRIMARY KEY, -- implies auto-increment in SQLite
    account_number TEXT NOT NULL UNIQUE,
    user_id INTEGER NOT NULL,
    balance REAL NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- SQLite uses TEXT for TIMESTAMP and DATETIME
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP, -- ON UPDATE CURRENT_TIMESTAMP is not directly supported by SQLite
    CONSTRAINT fk_account_user FOREIGN KEY(user_id) REFERENCES USER(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
"#;

/// SQL query to create the TRANSACTION table for SQLite.
pub const CREATE_TABLE_TRANSACTION: &str = r#"
CREATE TABLE TRANSX (
 	id INTEGER PRIMARY KEY, -- implies auto-increment in SQLite
 	account_number TEXT NOT NULL UNIQUE,
	seller TEXT NOT NULL,
	amount REAL, -- DECIMAL is typically mapped to REAL in SQLite
 	created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- SQLite uses TEXT for TIMESTAMP and DATETIME
 	updated_at TEXT DEFAULT CURRENT_TIMESTAMP, -- ON UPDATE CURRENT_TIMESTAMP is not directly supported by SQLite,
	CONSTRAINT fk_tx_account FOREIGN KEY(account_number) REFERENCES ACCOUNT(account_number)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);
"#;
