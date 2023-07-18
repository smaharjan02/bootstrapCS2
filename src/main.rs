mod bootstrap;
mod customer;
mod data_sampling;
mod nation;
mod orders;
#[allow(dead_code)]
mod parser;
mod region;
mod samples;

use rayon::prelude::*;

#[allow(unused_imports)]
use crate::{
    bootstrap::{
        bootstrap_sums, calculate_mean, calculate_variance, random_sample_with_replacement,
    },
    data_sampling::{create_sample, groundtruth, s1_sample_hashmap, S1Sample},
    parser::{parse_sql_query, Where},
    samples::{
        get_query_result, s2_sample_to_hashmap, s3_sample_to_hashmap, s4_sample_to_hashmap,
        s5_sample_to_hashmap, S2Sample, S3Sample,
    },
};
use crate::{
    customer::generate_s3_sample, nation::generate_s4_sample, orders::generate_s2_sample,
    region::generate_s5_sample,
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
        .expect("Sample fraction must be a valid floating-point number")
        / 100.0;

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
    let (join_conditions, filtered_conditions) = separate_conditions(where_conditions);

    // println!("Join Condition: {:#?}", join_conditions);
    // println!("Filtered Conditions: {:#?}", filtered_conditions);

    //creating connection to the database
    let conn = db_connection(db_file).unwrap();

    //running the query on the database to get the groundtruth
    let database_ground_truth = groundtruth(&conn, &query).unwrap();
    println!("Database Ground Truth: {}", database_ground_truth);

    let query_result: Vec<i64>;

    //based on the join condition we generate the responding samples.
    if join_conditions.iter().any(|condition| {
        condition.get_left() == "l_orderkey" && condition.get_right() == "o_orderkey"
    }) {
        let s1_samples = create_sample(&conn, sample_fraction).unwrap();
        let s2_samples = generate_s2_sample(&conn, &s1_samples).unwrap();

        if join_conditions.iter().any(|condition| {
            condition.get_left() == "o_custkey" && condition.get_right() == "c_custkey"
        }) {
            let s3_samples = generate_s3_sample(&conn, &s2_samples).unwrap();

            if join_conditions.iter().any(|condition| {
                condition.get_left() == "c_nationkey" && condition.get_right() == "n_nationkey"
            }) {
                let s4_samples = generate_s4_sample(&conn, &s3_samples).unwrap();

                if join_conditions.iter().any(|condition| {
                    condition.get_left() == "n_regionkey" && condition.get_right() == "r_regionkey"
                }) {
                    let s5_samples = generate_s5_sample(&conn, &s4_samples).unwrap();
                    // Work with s5_samples
                    let s5_hashmap = s5_sample_to_hashmap(&s5_samples);
                    query_result = get_query_result(&s5_hashmap, &filtered_conditions);
                } else {
                    // Work with s4_samples
                    let s4_hashmap = s4_sample_to_hashmap(&s4_samples);
                    query_result = get_query_result(&s4_hashmap, &filtered_conditions);
                }
            } else {
                // Work with s3_samples
                let s3_hashmap = s3_sample_to_hashmap(&s3_samples);
                query_result = get_query_result(&s3_hashmap, &filtered_conditions);
            }
        } else {
            // Work with s2_samples
            let s2_hashmap = s2_sample_to_hashmap(&s2_samples);
            query_result = get_query_result(&s2_hashmap, &filtered_conditions);
        }
    } else {
        let s1_samples = create_sample(&conn, sample_fraction).unwrap();
        // Work with s1_samples
        let s1_hashmap = s1_sample_hashmap(&s1_samples);
        query_result = get_query_result(&s1_hashmap, &filtered_conditions);
    }

    //calulating the sample ground truth
    let sum: i64 = query_result.par_iter().sum();
    let sample_ground_truth = sum as f64 / sample_fraction;
    println!("Sample Ground Truth: {}", sample_ground_truth);

    //resampling the query result with replacement
    let (bootstrap_sample, bootstrap_time_taken) =
        bootstrap_sums(&query_result, bootstrap_size, sample_fraction);
    // println!("Bootstrap Sample: {:#?}", bootstrap_sample);
    println!("Bootstrap Time Taken: {:.2}s", bootstrap_time_taken);

    let bootstrap_std_error = calculate_variance(&bootstrap_sample, bootstrap_size);
    //println!("Mean: {}", mean);
    println!("Standard Error: {:.2}", bootstrap_std_error);

    // z-score for 95% confidence level
    let z_score = 1.960;
    let ci = z_score * bootstrap_std_error;
    // println!("Margin of Error: {:.2}", ci);

    let lower_bound = sample_ground_truth as f64 - ci;
    let upper_bound = sample_ground_truth as f64 + ci;

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
