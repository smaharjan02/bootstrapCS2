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
    bootstrap::{
        bootstrap_sums, calculate_mean, calculate_std_error, random_sample_with_replacement,
    },
    customer::customer_data,
    data_sampling::{create_sample, groundtruth, s1_sample_hashmap, sample_ground_truth, S1Sample},
    orders::orders_data,
    parser::{parse_sql_query, Where},
    samples::{
        get_query_result, join_table, s2_sample_to_hashmap, s3_join, s3_sample_to_hashmap,
        S2Sample, S3Sample,
    },
};
use std::env;
use std::time::Instant;

//making the connection to the database
fn db_connection(db_file: &str) -> Result<rusqlite::Connection, rusqlite::Error> {
    let conn = rusqlite::Connection::open(db_file)?;
    Ok(conn)
}

fn read_query(file_path: &str) -> String {
    let query = std::fs::read_to_string(file_path).unwrap();
    query
}

/// Retrieves the value associated with a specific command-line flag.
/// Returns `None` if the flag is not found or if the value is missing.
fn get_argument_value<'a>(args: &'a [String], flag: &'a str) -> Option<&'a String> {
    args.iter()
        .position(|arg| arg == flag)
        .map(|i| &args[i + 1])
}

//function to seperate join_condtion and selection condition
fn separate_conditions(where_conditions: Vec<Where>) -> (Vec<Where>, Vec<Where>) {
    let join_conditions: Vec<Where> = where_conditions
        .clone()
        .into_iter()
        .filter(|condition| condition.get_operator() == "=")
        .collect();

    let selection_conditions: Vec<Where> = where_conditions
        .into_iter()
        .filter(|condition| condition.get_operator() == "<" || condition.get_operator() == ">")
        .collect();

    (join_conditions, selection_conditions)
}

fn main() {
    // Start timing
    let start = Instant::now();
    // Collect the command-line arguments into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // Verify that the required number of arguments is provided.
    if args.len() < 7 {
        // Print the usage message and exit the program with an error code.
        eprintln!(
            "Usage: {} -d <database> -s <sample_fraction> -b <bootstrap_num>",
            args[0]
        );
        std::process::exit(1);
    }

    // Retrieve the value associated with the "-d" flag, which represents the database file.
    let db_file = get_argument_value(&args, "-d").expect("Missing -d <database> argument");

    // Retrieve the value associated with the "-s" flag, which represents the sample fraction.
    let sample_fraction = get_argument_value(&args, "-s")
        .expect("Missing -s <sample_fraction> argument")
        .parse::<f64>()
        .expect("Sample fraction must be a valid floating-point number");

    // Retrieve the value associated with the "-b" flag, which represents the bootstrap size.
    let bootstrap_size = get_argument_value(&args, "-b")
        .expect("Missing -b <bootstrap_num> argument")
        .parse::<usize>()
        .expect("Bootstrap size must be a valid integer");

    //Parsing the sql query and saving it in a struct
    let query = read_query("query.txt");

    let (_, sql_query) = parse_sql_query(&query).unwrap();
    let select = sql_query.get_select();
    //println!("Tables: {:?}", select.get_table());

    let where_conditions = select.get_where_clause().clone().unwrap();
    // println!("Where Conditions: {:#?}", where_conditions);

    //seperating join conditions
    let (_join_conditions, filtered_conditions) = separate_conditions(where_conditions);

    //println!("Join Condition: {:#?}", join_conditions);
    // println!("Filtered Conditions: {:#?}", filtered_conditions);

    //creating connection to the database
    let conn = db_connection(db_file).unwrap();

    //running the query on the database to get the groundtruth
    let database_ground_truth = groundtruth(&conn, &query).unwrap();
    println!("Database Ground Truth: {}", database_ground_truth);

    //Using SRSWOR to create a sample of the LineItem table (S*1)
    let (s1_samples, _count) = create_sample(&conn, sample_fraction).unwrap();

    //getting all the data from the orders table
    let orders_data = orders_data(&conn).unwrap();

    //getting all the data from the customer table
    let customer_data = customer_data(&conn).unwrap();

    //Joining lineitem table with orders where l.orderkey = o.orderkey
    let s2_samples = join_table(&s1_samples, &orders_data);

    //joining the S2Sample with customer table where c.custkey = o.custkey
    let s3_samples = s3_join(&s2_samples, customer_data);

    let tables: Vec<String> = select
        .get_table()
        .iter()
        .flat_map(|table_str| table_str.split(',').map(String::from))
        .collect();

    let table_len = tables.len();

    let sample_query = match table_len {
        1 => {
            let hash_first_sample = s1_sample_hashmap(&s1_samples);
            //taking in the hashmap and where conditions to get the query result as 1 or 0
            get_query_result(&hash_first_sample, &filtered_conditions)
        }
        2 => {
            //getting the join sample of S1Sample and S2Sample
            let hash_first_join = s2_sample_to_hashmap(&s2_samples);
            //taking in the hashmap and where conditions to get the query result as 1 or 0
            get_query_result(&hash_first_join, &filtered_conditions)
        }
        3 => {
            //getting the join sample of S3Sample as it has relation from s1sample ,s2sample
            //converting the join sample to hashmap
            let hashed_second_join = s3_sample_to_hashmap(&s3_samples);
            //taking in the hashmap and where conditions to get the query result as 1 or 0
            get_query_result(&hashed_second_join, &filtered_conditions)
        }
        _ => {
            println!("Unexpected table length.");
            // Return an empty vector of i64
            Vec::new()
        }
    };

    // println!("Sample Query: {:#?}", sample_query);

    //calulating the sample ground truth
    let sample_ground_truth = sample_ground_truth(&sample_query, sample_fraction);
    println!("Sample Ground Truth: {}", sample_ground_truth);

    //resampling the query result with replacement
    let (bootstrap_sample, bootstrap_time_taken) =
        bootstrap_sums(&sample_query, bootstrap_size, sample_fraction);
    // println!("Bootstrap Sample: {:#?}", bootstrap_sample);
    println!("Bootstrap Time Taken: {:.2}s", bootstrap_time_taken);

    let mean = calculate_mean(&bootstrap_sample);
    let bootstrap_std_error = calculate_std_error(&bootstrap_sample, mean);
    // println!("Mean: {}", mean);
    println!("Standard Error: {:.2}", bootstrap_std_error);

    // z-score for 95% confidence level
    let z_score = 1.960;
    let margin_of_error = z_score * bootstrap_std_error;
    // println!("Margin of Error: {:.2}", margin_of_error);

    let lower_bound = sample_ground_truth as f64 - margin_of_error;
    let upper_bound = sample_ground_truth as f64 + margin_of_error;

    println!("CI: [{:.2}, {:.2}]", lower_bound, upper_bound);

    if (database_ground_truth as f64) >= lower_bound
        && (database_ground_truth as f64) <= upper_bound
    {
        println!(
            "The database ground truth {} is within the confidence interval",
            database_ground_truth
        );
    } else {
        println!(
            "The database ground truth {} is not within the confidence interval",
            database_ground_truth
        )
    }
    // End timing
    let duration = start.elapsed().as_secs_f64();

    // Print the elapsed time in seconds
    println!("Execution time: {:.2}s", duration);
}
