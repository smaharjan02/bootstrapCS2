use rusqlite::{Connection, Result, Row};
use std::error::Error;
#[derive(Debug, Clone)]
pub struct Orders {
    pub o_orderkey: i32,
    pub o_custkey: i32,
    pub o_orderstatus: String,
    pub o_totalprice: f64,
    pub o_orderdate: String,
    pub o_orderpriority: String,
    pub o_clerk: String,
    pub o_shippriority: i32,
    pub o_comment: String,
}

impl Orders {
    fn from_row(row: &Row) -> Result<Self> {
        Ok(Orders {
            o_orderkey: row.get(0)?,
            o_custkey: row.get(1)?,
            o_orderstatus: row.get(2)?,
            o_totalprice: row.get(3)?,
            o_orderdate: row.get(4)?,
            o_orderpriority: row.get(5)?,
            o_clerk: row.get(6)?,
            o_shippriority: row.get(7)?,
            o_comment: row.get(8)?,
        })
    }
}

// pub fn orders_data(conn:&Connection) -> Result<Vec<Orders>, Box<dyn Error>> {
//     // Define the SQL query to retrieve all rows from the orders table
//     let query = "SELECT * FROM orders;" ;

//     // Execute the query and get all the rows
//     let mut stmt = conn.prepare(query)?;
//     let all_rows = stmt.query_map([], Orders::from_row)?.collect::<Result<Vec<Orders>, _>>()?;

//     Ok(all_rows)

// }

pub fn orders_data(conn: &Connection) -> Result<Vec<Orders>, Box<dyn Error>> {
    // Define the SQL query to retrieve all rows from the orders table
    let query = "SELECT * FROM orders;";

    // Execute the query and get a streaming iterator
    let mut stmt = conn.prepare(query)?;
    let stream = stmt.query_map([], Orders::from_row)?;

    // Collect the streamed data into a vector
    let all_rows = stream.collect::<Result<Vec<Orders>, _>>()?;

    Ok(all_rows)
}
