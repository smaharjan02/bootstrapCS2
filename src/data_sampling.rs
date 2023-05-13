use std::collections::HashMap;

// use std:: {collections::HashMap};
use rand::seq::IteratorRandom;
use rusqlite::{Connection, Result};

//s*1 and the lineitem are same (s*1 is the sample(SRSWOR) of lineitem)
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct S1Sample {
    pub l_orderkey: i32,
    pub l_partkey: i32,
    pub l_suppkey: i32,
    pub l_linenumber: i32,
    pub l_quantity: f64,
    pub l_extendedprice: f64,
    pub l_discount: f64,
    pub l_tax: f64,
    pub l_returnflag: String,
    pub l_linestatus: String,
    pub l_shipdate: String,
    pub l_commitdate: String,
    pub l_receiptdate: String,
    pub l_shipinstruct: String,
    pub l_shipmode: String,
    pub l_comment: String,
}

impl S1Sample {
    fn from_row(row: &rusqlite::Row) -> Result<Self> {
        Ok(S1Sample {
            l_orderkey: row.get(0)?,
            l_partkey: row.get(1)?,
            l_suppkey: row.get(2)?,
            l_linenumber: row.get(3)?,
            l_quantity: row.get(4)?,
            l_extendedprice: row.get(5)?,
            l_discount: row.get(6)?,
            l_tax: row.get(7)?,
            l_returnflag: row.get(8)?,
            l_linestatus: row.get(9)?,
            l_shipdate: row.get(10)?,
            l_commitdate: row.get(11)?,
            l_receiptdate: row.get(12)?,
            l_shipinstruct: row.get(13)?,
            l_shipmode: row.get(14)?,
            l_comment: row.get(15)?,
        })
    }

    //getter and setter
}

pub fn create_sample(
    conn: &Connection,
    sample_fraction: f64,
) -> Result<Vec<S1Sample>, Box<dyn std::error::Error>> {
    // // Open the SQLite database connection
    // let conn = Connection::open(db_file)?;

    // Define the SQL query to retrieve all rows from the lineitem table
    let query = "SELECT * FROM lineitem;";

    // Execute the query and get all the rows
    let mut stmt = conn.prepare(query)?;
    let all_rows = stmt
        .query_map([], S1Sample::from_row)?
        .collect::<Result<Vec<S1Sample>, _>>()?;

    // Calculate the sample size
    let sample_size = (all_rows.len() as f64 * sample_fraction).floor() as usize;

    // Randomly select the sample without replacement
    let mut rng = rand::thread_rng();
    let sample = all_rows
        .iter()
        .cloned()
        .choose_multiple(&mut rng, sample_size);

    // Close the database connection
    drop(stmt);
    drop(conn);

    // // Calculate the ground truth of the sample and database
    // let sample_ground_truth = sample.iter().map(|row| row.l_quantity).sum::<f64>();
    // let database_ground_truth = all_rows.iter().map(|row| row.l_quantity).sum::<f64>();

    Ok(sample)
}

//hashmap for S*1 Sample with SRSWOR
pub fn s1_sample_hashmap(lineitems: &[S1Sample]) -> Vec<HashMap<String, String>> {
    let mut hashmaps = Vec::new();

    for sample in lineitems {
        let mut hashmap = HashMap::new();

        hashmap.insert("l_orderkey".to_string(), sample.l_orderkey.to_string());
        hashmap.insert("l_partkey".to_string(), sample.l_partkey.to_string());
        hashmap.insert("l_suppkey".to_string(), sample.l_suppkey.to_string());
        hashmap.insert("l_linenumber".to_string(), sample.l_linenumber.to_string());
        hashmap.insert("l_quantity".to_string(), sample.l_quantity.to_string());
        hashmap.insert(
            "l_extendedprice".to_string(),
            sample.l_extendedprice.to_string(),
        );
        hashmap.insert("l_discount".to_string(), sample.l_discount.to_string());
        hashmap.insert("l_tax".to_string(), sample.l_tax.to_string());
        hashmap.insert("l_returnflag".to_string(), sample.l_returnflag.clone());
        hashmap.insert("l_linestatus".to_string(), sample.l_linestatus.clone());
        hashmap.insert("l_shipdate".to_string(), sample.l_shipdate.clone());
        hashmap.insert("l_commitdate".to_string(), sample.l_commitdate.clone());
        hashmap.insert("l_receiptdate".to_string(), sample.l_receiptdate.clone());
        hashmap.insert("l_shipinstruct".to_string(), sample.l_shipinstruct.clone());
        hashmap.insert("l_shipmode".to_string(), sample.l_shipmode.clone());
        hashmap.insert("l_comment".to_string(), sample.l_comment.clone());

        hashmaps.push(hashmap);
    }

    hashmaps
}
