
use rusqlite::{Connection, Result,Row};
use std::error::Error;

#[derive(Debug,Clone)]
pub struct Customer {
    pub c_custkey: i32,
    pub c_name: String,
    pub c_address: String,
    pub c_nationkey: i32,
    pub c_phone: String,
    pub c_acctbal: f64,
    pub c_mktsegment: String,
    pub c_comment: String,
}

impl Customer {
    fn from_row(row: &Row) -> Result<Self> {
        Ok(Customer {
            c_custkey: row.get(0)?,
            c_name: row.get(1)?,
            c_address: row.get(2)?,
            c_nationkey: row.get(3)?,
            c_phone: row.get(4)?,
            c_acctbal: row.get(5)?,
            c_mktsegment: row.get(6)?,
            c_comment: row.get(7)?,
        })
    }
}

pub fn customer_data(conn: &Connection) -> Result<Vec<Customer>, Box<dyn Error>> {
    // Define the SQL query to retrieve all rows from the customer table
    let query = "SELECT * FROM customer;";

    // Execute the query and get all the rows
    let mut stmt = conn.prepare(query)?;
    let all_rows = stmt.query_map([], Customer::from_row)?.collect::<Result<Vec<Customer>, _>>()?;

    Ok(all_rows)
}

