mod queries;

use queries::{CREATE_TABLE_ACCOUNT, CREATE_TABLE_TRANSACTION, CREATE_TABLE_USER};
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(CREATE_TABLE_USER, ());
    conn.execute(CREATE_TABLE_ACCOUNT, ());
    conn.execute(CREATE_TABLE_TRANSACTION, ());
    Ok(())
}
