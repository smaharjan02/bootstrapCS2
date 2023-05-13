use std::collections::HashMap;

use crate::{customer::Customer, data_sampling::S1Sample, orders::Orders};

#[allow(dead_code)]
#[derive(Debug)]
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

#[allow(dead_code)]
#[derive(Debug)]
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

//Function to do a hashmap with the S_LineItem table and orders table to create a S2_sample
// Define the LineItem, Order, and S2Sample structs with their fields
// Make sure these structs are defined correctly with public fields

pub fn join_table(sample: &Vec<S1Sample>, orders: Vec<Orders>) -> Vec<S2Sample> {
    let mut s2_sample: Vec<S2Sample> = Vec::new();
    let mut orders_map: HashMap<i32, Orders> = HashMap::new();

    for order in orders {
        orders_map.insert(order.o_orderkey, order);
    }

    for lineitem in sample {
        if let Some(order) = orders_map.get(&lineitem.l_orderkey) {
            s2_sample.push(S2Sample {
                //Join key
                o_orderkey: lineitem.l_orderkey,

                //LineItem fields
                l_partkey: lineitem.l_partkey,
                l_suppkey: lineitem.l_suppkey,
                l_linenumber: lineitem.l_linenumber,
                l_quantity: lineitem.l_quantity,
                l_extendedprice: lineitem.l_extendedprice,
                l_discount: lineitem.l_discount,
                l_tax: lineitem.l_tax,
                l_returnflag: lineitem.l_returnflag.clone(),
                l_linestatus: lineitem.l_linestatus.clone(),
                l_shipdate: lineitem.l_shipdate.clone(),
                l_commitdate: lineitem.l_commitdate.clone(),
                l_receiptdate: lineitem.l_receiptdate.clone(),
                l_shipinstruct: lineitem.l_shipinstruct.clone(),
                l_shipmode: lineitem.l_shipmode.clone(),
                l_comment: lineitem.l_comment.clone(),

                //Orders fields
                o_custkey: order.o_custkey,
                o_orderstatus: order.o_orderstatus.clone(),
                o_totalprice: order.o_totalprice,
                o_orderdate: order.o_orderdate.clone(),
                o_orderpriority: order.o_orderpriority.clone(),
                o_clerk: order.o_clerk.clone(),
                o_shippriority: order.o_shippriority,
                o_comment: order.o_comment.clone(),
            });
        }
    }
    s2_sample
}

// //Function to convert the s2samples into a hashmap so that we can filter based on the selection condition
// pub fn s2_sample_to_hashmap(samples: &[S2Sample]) -> Vec<HashMap<String, String>> {
//     let mut hashmaps = Vec::new();

//     for sample in samples {
//         let mut hashmap = HashMap::new();

//         hashmap.insert("o_orderkey".to_string(), sample.orderkey.to_string());
//         hashmap.insert("l_partkey".to_string(), sample.l_partkey.to_string());
//         hashmap.insert("l_suppkey".to_string(), sample.l_suppkey.to_string());
//         hashmap.insert("l_linenumber".to_string(), sample.l_linenumber.to_string());
//         hashmap.insert("l_quantity".to_string(), sample.l_quantity.to_string());
//         hashmap.insert(
//             "l_extendedprice".to_string(),
//             sample.l_extendedprice.to_string(),
//         );
//         hashmap.insert("l_discount".to_string(), sample.l_discount.to_string());
//         hashmap.insert("l_tax".to_string(), sample.l_tax.to_string());
//         hashmap.insert("l_returnflag".to_string(), sample.l_returnflag.clone());
//         hashmap.insert("l_linestatus".to_string(), sample.l_linestatus.clone());
//         hashmap.insert("l_shipdate".to_string(), sample.l_shipdate.clone());
//         hashmap.insert("l_commitdate".to_string(), sample.l_commitdate.clone());
//         hashmap.insert("l_receiptdate".to_string(), sample.l_receiptdate.clone());
//         hashmap.insert("l_shipinstruct".to_string(), sample.l_shipinstruct.clone());
//         hashmap.insert("l_shipmode".to_string(), sample.l_shipmode.clone());
//         hashmap.insert("l_comment".to_string(), sample.l_comment.clone());
//         hashmap.insert("o_custkey".to_string(), sample.o_custkey.to_string());
//         hashmap.insert("o_orderstatus".to_string(), sample.o_orderstatus.clone());
//         hashmap.insert("o_totalprice".to_string(), sample.o_totalprice.to_string());
//         hashmap.insert("o_orderdate".to_string(), sample.o_orderdate.clone());
//         hashmap.insert(
//             "o_orderpriority".to_string(),
//             sample.o_orderpriority.clone(),
//         );
//         hashmap.insert("o_clerk".to_string(), sample.o_clerk.clone());
//         hashmap.insert(
//             "o_shippriority".to_string(),
//             sample.o_shippriority.to_string(),
//         );
//         hashmap.insert("o_comment".to_string(), sample.o_comment.clone());

//         hashmaps.push(hashmap);
//     }

//     hashmaps
// }

//function to create s3sample using s2sample and customer table
pub fn s3_join(s2samples: &Vec<S2Sample>, customers: Vec<Customer>) -> Vec<S3Sample> {
    let mut s3_sample: Vec<S3Sample> = Vec::new();
    let mut customer_map: HashMap<i32, Customer> = HashMap::new();

    for customer in customers {
        customer_map.insert(customer.c_custkey, customer);
    }

    for s2sample in s2samples {
        if let Some(customer) = customer_map.get(&s2sample.o_custkey) {
            s3_sample.push(S3Sample {
                //Join key
                c_custkey: s2sample.o_custkey,
                o_orderkey: s2sample.o_orderkey,

                //LineItem fields
                l_partkey: s2sample.l_partkey,
                l_suppkey: s2sample.l_suppkey,
                l_linenumber: s2sample.l_linenumber,
                l_quantity: s2sample.l_quantity,
                l_extendedprice: s2sample.l_extendedprice,
                l_discount: s2sample.l_discount,
                l_tax: s2sample.l_tax,
                l_returnflag: s2sample.l_returnflag.clone(),
                l_linestatus: s2sample.l_linestatus.clone(),
                l_shipdate: s2sample.l_shipdate.clone(),
                l_commitdate: s2sample.l_commitdate.clone(),
                l_receiptdate: s2sample.l_receiptdate.clone(),
                l_shipinstruct: s2sample.l_shipinstruct.clone(),
                l_shipmode: s2sample.l_shipmode.clone(),
                l_comment: s2sample.l_comment.clone(),

                //Orders fields
                o_orderstatus: s2sample.o_orderstatus.clone(),
                o_totalprice: s2sample.o_totalprice,
                o_orderdate: s2sample.o_orderdate.clone(),
                o_orderpriority: s2sample.o_orderpriority.clone(),
                o_clerk: s2sample.o_clerk.clone(),
                o_shippriority: s2sample.o_shippriority,
                o_comment: s2sample.o_comment.clone(),

                //Customer fields
                c_name: customer.c_name.clone(),
                c_address: customer.c_address.clone(),
                c_nationkey: customer.c_nationkey,
                c_phone: customer.c_phone.clone(),
                c_acctbal: customer.c_acctbal,
                c_mktsegment: customer.c_mktsegment.clone(),
                c_comment: customer.c_comment.clone(),
            });
        }
    }
    s3_sample
}

// //function to create a hashmap of s3sample so that we can filter based on the selection condition
// pub fn s3_sample_to_hashmap(samples: &[S3Sample]) -> Vec<HashMap<String, String>> {
//     let mut hashmaps = Vec::new();

//     for sample in samples {
//         let mut hashmap = HashMap::new();

//         hashmap.insert("c_custkey".to_string(), sample.custkey.to_string());
//         hashmap.insert("o_orderkey".to_string(), sample.orderkey.to_string());
//         hashmap.insert("l_partkey".to_string(), sample.l_partkey.to_string());
//         hashmap.insert("l_suppkey".to_string(), sample.l_suppkey.to_string());
//         hashmap.insert("l_linenumber".to_string(), sample.l_linenumber.to_string());
//         hashmap.insert("l_quantity".to_string(), sample.l_quantity.to_string());
//         hashmap.insert(
//             "l_extendedprice".to_string(),
//             sample.l_extendedprice.to_string(),
//         );
//         hashmap.insert("l_discount".to_string(), sample.l_discount.to_string());
//         hashmap.insert("l_tax".to_string(), sample.l_tax.to_string());
//         hashmap.insert("l_returnflag".to_string(), sample.l_returnflag.clone());
//         hashmap.insert("l_linestatus".to_string(), sample.l_linestatus.clone());
//         hashmap.insert("l_shipdate".to_string(), sample.l_shipdate.clone());
//         hashmap.insert("l_commitdate".to_string(), sample.l_commitdate.clone());
//         hashmap.insert("l_receiptdate".to_string(), sample.l_receiptdate.clone());
//         hashmap.insert("l_shipinstruct".to_string(), sample.l_shipinstruct.clone());
//         hashmap.insert("l_shipmode".to_string(), sample.l_shipmode.clone());
//         hashmap.insert("l_comment".to_string(), sample.l_comment.clone());
//         hashmap.insert("o_orderstatus".to_string(), sample.o_orderstatus.clone());
//         hashmap.insert("o_totalprice".to_string(), sample.o_totalprice.to_string());
//         hashmap.insert("o_orderdate".to_string(), sample.o_orderdate.clone());
//         hashmap.insert(
//             "o_orderpriority".to_string(),
//             sample.o_orderpriority.clone(),
//         );
//         hashmap.insert("o_clerk".to_string(), sample.o_clerk.clone());
//         hashmap.insert(
//             "o_shippriority".to_string(),
//             sample.o_shippriority.to_string(),
//         );
//         hashmap.insert("o_comment".to_string(), sample.o_comment.clone());
//         hashmap.insert("c_name".to_string(), sample.c_name.clone());
//         hashmap.insert("c_address".to_string(), sample.c_address.clone());
//         hashmap.insert("c_nationkey".to_string(), sample.c_nationkey.to_string());
//         hashmap.insert("c_phone".to_string(), sample.c_phone.clone());
//         hashmap.insert("c_acctbal".to_string(), sample.c_acctbal.to_string());
//         hashmap.insert("c_mktsegment".to_string(), sample.c_mktsegment.clone());
//         hashmap.insert("c_comment".to_string(), sample.c_comment.clone());

//         hashmaps.push(hashmap);
//     }

//     hashmaps
// }
