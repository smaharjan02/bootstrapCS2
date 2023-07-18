use crate::parser::Where;
use std::collections::HashMap;
use std::time::Instant;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct S2Sample {
    //Join key
    pub o_orderkey: i32,

    //LineItem fields
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

    //Orders fields
    pub o_custkey: i32,
    pub o_orderstatus: String,
    pub o_totalprice: f64,
    pub o_orderdate: String,
    pub o_orderpriority: String,
    pub o_clerk: String,
    pub o_shippriority: i32,
    pub o_comment: String,
}

pub fn s2_sample_to_hashmap(samples: &[S2Sample]) -> Vec<HashMap<String, String>> {
    let start_time = Instant::now(); // Start measuring time
    let hashmaps = samples
        .iter()
        .map(|sample| {
            let mut hashmap = HashMap::new();
            hashmap.insert("o_orderkey".to_string(), sample.o_orderkey.to_string());
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
            hashmap.insert("o_custkey".to_string(), sample.o_custkey.to_string());
            hashmap.insert("o_orderstatus".to_string(), sample.o_orderstatus.clone());
            hashmap.insert("o_totalprice".to_string(), sample.o_totalprice.to_string());
            hashmap.insert("o_orderdate".to_string(), sample.o_orderdate.clone());
            hashmap.insert(
                "o_orderpriority".to_string(),
                sample.o_orderpriority.clone(),
            );
            hashmap.insert("o_clerk".to_string(), sample.o_clerk.clone());
            hashmap.insert(
                "o_shippriority".to_string(),
                sample.o_shippriority.to_string(),
            );
            hashmap.insert("o_comment".to_string(), sample.o_comment.clone());
            hashmap
        })
        .collect();

    let end_time = Instant::now(); // Stop measuring time
    let _execution_time = end_time - start_time;

    // println!(
    //     "Execution time s2_sample_to_hashmap: {:.3}",
    //     execution_time.as_secs_f64()
    // );

    hashmaps
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct S3Sample {
    //Join key
    pub c_custkey: i32,
    pub o_orderkey: i32,

    //LineItem fields
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

    //Orders fields
    pub o_orderstatus: String,
    pub o_totalprice: f64,
    pub o_orderdate: String,
    pub o_orderpriority: String,
    pub o_clerk: String,
    pub o_shippriority: i32,
    pub o_comment: String,

    //Customer fields
    pub c_name: String,
    pub c_address: String,
    pub c_nationkey: i32,
    pub c_phone: String,
    pub c_acctbal: f64,
    pub c_mktsegment: String,
    pub c_comment: String,
}

// //function to create a hashmap of s3sample so that we can filter based on the selection condition
pub fn s3_sample_to_hashmap(samples: &[S3Sample]) -> Vec<HashMap<String, String>> {
    let start_time = Instant::now(); // Start measuring time

    let mut hashmaps = Vec::new();

    for sample in samples {
        let mut hashmap = HashMap::new();

        hashmap.insert("c_custkey".to_string(), sample.c_custkey.to_string());
        hashmap.insert("o_orderkey".to_string(), sample.o_orderkey.to_string());
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
        hashmap.insert("o_orderstatus".to_string(), sample.o_orderstatus.clone());
        hashmap.insert("o_totalprice".to_string(), sample.o_totalprice.to_string());
        hashmap.insert("o_orderdate".to_string(), sample.o_orderdate.clone());
        hashmap.insert(
            "o_orderpriority".to_string(),
            sample.o_orderpriority.clone(),
        );
        hashmap.insert("o_clerk".to_string(), sample.o_clerk.clone());
        hashmap.insert(
            "o_shippriority".to_string(),
            sample.o_shippriority.to_string(),
        );
        hashmap.insert("o_comment".to_string(), sample.o_comment.clone());
        hashmap.insert("c_name".to_string(), sample.c_name.clone());
        hashmap.insert("c_address".to_string(), sample.c_address.clone());
        hashmap.insert("c_nationkey".to_string(), sample.c_nationkey.to_string());
        hashmap.insert("c_phone".to_string(), sample.c_phone.clone());
        hashmap.insert("c_acctbal".to_string(), sample.c_acctbal.to_string());
        hashmap.insert("c_mktsegment".to_string(), sample.c_mktsegment.clone());
        hashmap.insert("c_comment".to_string(), sample.c_comment.clone());

        hashmaps.push(hashmap);
    }
    let end_time = Instant::now(); // Stop measuring time
    let _execution_time = end_time - start_time;

    // println!(
    //     "Execution time s3_sample_to_hashmap: {:.3}",
    //     execution_time.as_secs_f64()
    // );

    hashmaps
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct S4Sample {
    //Join key
    pub n_nationkey: i32,
    pub c_custkey: i32,
    pub o_orderkey: i32,

    //LineItem fields
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

    //Orders fields
    pub o_orderstatus: String,
    pub o_totalprice: f64,
    pub o_orderdate: String,
    pub o_orderpriority: String,
    pub o_clerk: String,
    pub o_shippriority: i32,
    pub o_comment: String,

    //Customer fields
    pub c_name: String,
    pub c_address: String,
    pub c_nationkey: i32,
    pub c_phone: String,
    pub c_acctbal: f64,
    pub c_mktsegment: String,
    pub c_comment: String,

    //Nation fileds
    pub n_name: String,
    pub n_regionkey: i32,
    pub n_comment: String,
}

//s4sample to hashmap for faster searching
pub fn s4_sample_to_hashmap(samples: &[S4Sample]) -> Vec<HashMap<String, String>> {
    let start_time = Instant::now(); // Start measuring time

    let mut hashmaps = Vec::new();

    for sample in samples {
        let mut hashmap = HashMap::new();
        hashmap.insert("n_nationkey".to_string(), sample.n_nationkey.to_string());
        hashmap.insert("c_custkey".to_string(), sample.c_custkey.to_string());
        hashmap.insert("o_orderkey".to_string(), sample.o_orderkey.to_string());
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
        hashmap.insert("o_orderstatus".to_string(), sample.o_orderstatus.clone());
        hashmap.insert("o_totalprice".to_string(), sample.o_totalprice.to_string());
        hashmap.insert("o_orderdate".to_string(), sample.o_orderdate.clone());
        hashmap.insert(
            "o_orderpriority".to_string(),
            sample.o_orderpriority.clone(),
        );
        hashmap.insert("o_clerk".to_string(), sample.o_clerk.clone());
        hashmap.insert(
            "o_shippriority".to_string(),
            sample.o_shippriority.to_string(),
        );
        hashmap.insert("o_comment".to_string(), sample.o_comment.clone());
        hashmap.insert("c_name".to_string(), sample.c_name.clone());
        hashmap.insert("c_address".to_string(), sample.c_address.clone());
        hashmap.insert("c_nationkey".to_string(), sample.c_nationkey.to_string());
        hashmap.insert("c_phone".to_string(), sample.c_phone.clone());
        hashmap.insert("c_acctbal".to_string(), sample.c_acctbal.to_string());
        hashmap.insert("c_mktsegment".to_string(), sample.c_mktsegment.clone());
        hashmap.insert("c_comment".to_string(), sample.c_comment.clone());

        hashmap.insert("n_name".to_string(), sample.n_name.clone());
        hashmap.insert("n_regionkey".to_string(), sample.n_regionkey.to_string());
        hashmap.insert("n_comment".to_string(), sample.n_comment.clone());
        hashmaps.push(hashmap);
    }
    let end_time = Instant::now(); // Stop measuring time
    let _execution_time = end_time - start_time;

    // println!(
    //     "Execution time s4_sample_to_hashmap: {:.3}",
    //     execution_time.as_secs_f64()
    // );

    hashmaps
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct S5Sample {
    //Join key
    pub r_regionkey: i32,
    pub n_nationkey: i32,
    pub c_custkey: i32,
    pub o_orderkey: i32,

    //LineItem fields
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

    //Orders fields
    pub o_orderstatus: String,
    pub o_totalprice: f64,
    pub o_orderdate: String,
    pub o_orderpriority: String,
    pub o_clerk: String,
    pub o_shippriority: i32,
    pub o_comment: String,

    //Customer fields
    pub c_name: String,
    pub c_address: String,
    pub c_nationkey: i32,
    pub c_phone: String,
    pub c_acctbal: f64,
    pub c_mktsegment: String,
    pub c_comment: String,

    //Nation fileds
    pub n_name: String,
    pub n_regionkey: i32,
    pub n_comment: String,

    //Region Fields
    pub r_name: String,
    pub r_comment: String,
}

//s5sample to hashmap for faster searching
pub fn s5_sample_to_hashmap(samples: &[S5Sample]) -> Vec<HashMap<String, String>> {
    let start_time = Instant::now(); // Start measuring time

    let mut hashmaps = Vec::new();

    for sample in samples {
        let mut hashmap = HashMap::new();
        hashmap.insert("r_regionkey".to_string(), sample.r_regionkey.to_string());
        hashmap.insert("n_nationkey".to_string(), sample.n_nationkey.to_string());
        hashmap.insert("c_custkey".to_string(), sample.c_custkey.to_string());
        hashmap.insert("o_orderkey".to_string(), sample.o_orderkey.to_string());
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
        hashmap.insert("o_orderstatus".to_string(), sample.o_orderstatus.clone());
        hashmap.insert("o_totalprice".to_string(), sample.o_totalprice.to_string());
        hashmap.insert("o_orderdate".to_string(), sample.o_orderdate.clone());
        hashmap.insert(
            "o_orderpriority".to_string(),
            sample.o_orderpriority.clone(),
        );
        hashmap.insert("o_clerk".to_string(), sample.o_clerk.clone());
        hashmap.insert(
            "o_shippriority".to_string(),
            sample.o_shippriority.to_string(),
        );
        hashmap.insert("o_comment".to_string(), sample.o_comment.clone());
        hashmap.insert("c_name".to_string(), sample.c_name.clone());
        hashmap.insert("c_address".to_string(), sample.c_address.clone());
        hashmap.insert("c_nationkey".to_string(), sample.c_nationkey.to_string());
        hashmap.insert("c_phone".to_string(), sample.c_phone.clone());
        hashmap.insert("c_acctbal".to_string(), sample.c_acctbal.to_string());
        hashmap.insert("c_mktsegment".to_string(), sample.c_mktsegment.clone());
        hashmap.insert("c_comment".to_string(), sample.c_comment.clone());

        hashmap.insert("n_name".to_string(), sample.n_name.clone());
        hashmap.insert("n_regionkey".to_string(), sample.n_regionkey.to_string());
        hashmap.insert("n_comment".to_string(), sample.n_comment.clone());
        hashmaps.push(hashmap);
    }
    let end_time = Instant::now(); // Stop measuring time
    let _execution_time = end_time - start_time;

    // println!(
    //     "Execution time s5_sample_to_hashmap: {:.3}",
    //     execution_time.as_secs_f64()
    // );

    hashmaps
}

//fn to check for the where condition and return 1 if true or 0
pub fn get_query_result(data: &Vec<HashMap<String, String>>, conditions: &Vec<Where>) -> Vec<i64> {
    let mut results = Vec::with_capacity(data.len());

    for row in data {
        let mut all_conditions_passed = true;
        for condition in conditions {
            let column_value = match row.get(&condition.get_left().to_lowercase()) {
                Some(value) => value.parse::<f64>().unwrap_or(0.0), // parsing string as f64 so that it works on both int and float data types
                None => continue,
            };
            let condition_value = condition.get_right().parse::<f64>().unwrap_or(0.0);
            //matching the comparator and returning true or false based on the condition values
            let condition_result = match condition.get_operator() {
                "<" => column_value < condition_value,
                ">" => column_value > condition_value,
                _ => false,
            };

            all_conditions_passed &= condition_result;
        }

        //inserting 1 if all conditions are true else inserting 0
        let result = if all_conditions_passed { 1 } else { 0 };
        results.push(result);
    }

    results
}
