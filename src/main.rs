#[allow(dead_code)]
mod bootstrap;
#[allow(dead_code)]
mod customer;
#[allow(dead_code)]
mod data_sampling;
#[allow(dead_code)]
mod orders;
#[allow(dead_code)]
mod parser;

mod samples;

#[allow(unused_imports)]
use crate::{
    bootstrap::resampling,
    customer::customer_data,
    data_sampling::create_sample,
    orders::orders_data,
    parser::{parse_sql_query, Where},
    samples::{join_table, s3_join},
};

//making the connection to the database
fn db_connection(db_file: &str) -> Result<rusqlite::Connection, rusqlite::Error> {
    let conn = rusqlite::Connection::open(db_file)?;
    Ok(conn)
}

fn read_query(file_path: &str) -> String {
    let query = std::fs::read_to_string(file_path).unwrap();
    query
}

fn main() {
    //Parsing the sql query and saving it in a struct
    let query = read_query("query.txt");

    let (_, sql_query) = parse_sql_query(&query).unwrap();
    let select = sql_query.get_select();
    println!("Function: {}", select.get_function());
    println!("Tables: {:?}", select.get_table());

    let where_conditions = select.get_where_clause().clone().unwrap();
    // println!("Where Conditions: {:#?}", where_conditions);

    let filtered_conditions: Vec<Where> = where_conditions
        .into_iter()
        .filter(|condition| {
            let left = condition.get_left();
            left != "l_orderkey" && left != "o_orderkey" && left != "o_custkey"
        })
        .collect();

    println!("Filtered Conditions: {:#?}", filtered_conditions);

    //database and sample_fraction
    let db_file = "tpch100m.db";
    let sample_fraction = 0.001;

    //creating connection to the database
    let conn = db_connection(db_file).unwrap();

    //Using SRSWOR to create a sample of the LineItem table (S*1)
    let s1_samples = create_sample(&conn, sample_fraction).unwrap();
    println!("Total records in S1sample: {:#?}", s1_samples.len());

    //getting all the data from the orders table
    let orders_data = orders_data(&conn).unwrap();
    // println!("Total record of data in orders: {:#?}", orders_data.len());

    //getting all the data from the customer table
    let customer_data = customer_data(&conn).unwrap();
    // println!(
    //     "Total record of data in customer: {:#?}",
    //     customer_data.len()
    // );

    //Joining lineitem table with orders where l.orderkey = o.orderkey
    let s2_samples = join_table(&s1_samples, orders_data);
    println!("Total record of data in S2Sample: {:#?}", s2_samples.len());

    //joining the S2Sample with customer table where c.custkey = o.custkey
    let s3_samples = s3_join(&s2_samples, customer_data);
    println!("Total record of data in S3Sample: {:#?}", s3_samples.len());

    let tables: Vec<String> = select
        .get_table()
        .iter()
        .flat_map(|table_str| table_str.split(',').map(String::from))
        .collect();

    let table_len = tables.len();

    match table_len {
        1 => {
            let table1 = tables[0].clone();
            println!("Table 1: {}", table1);
        }
        2 => {
            let table1 = tables[0].clone();
            let table2 = tables[1].clone();
            println!("Table 1: {}", table1);
            println!("Table 2: {}", table2);
        }
        3 => {
            let table1 = tables[0].clone();
            let table2 = tables[1].clone();
            let table3 = tables[2].clone();
            println!("Table 1: {}", table1);
            println!(); // Add an empty line
            println!("Table 2: {}", table2);
            println!(); // Add an empty line
            println!("Table 3: {}", table3);
        }
        _ => panic!("Unsupported number of tables."),
    }
}
