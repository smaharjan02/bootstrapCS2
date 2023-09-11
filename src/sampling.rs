use rusqlite::{params, Connection, Result};

pub fn create_sample_tables(conn: &Connection, sample_fraction: f64) -> Result<()> {
    // Drop existing sample tables if they exist
    conn.execute("DROP TABLE IF EXISTS s1_sample", params![])?;
    conn.execute("DROP TABLE IF EXISTS s2_sample", params![])?;
    conn.execute("DROP TABLE IF EXISTS s3_sample", params![])?;
    conn.execute("DROP TABLE IF EXISTS s4_sample", params![])?;
    conn.execute("DROP TABLE IF EXISTS s5_sample", params![])?;

    // Create an empty s1_sample table with the same structure as lineitem
    conn.execute(
        "CREATE TABLE s1_sample AS SELECT * FROM lineitem WHERE 1=0",
        params![],
    )?;

    // Generate a temporary table with shuffled row IDs from the lineitem table
    conn.execute(
        "CREATE TEMP TABLE ids AS SELECT rowid FROM lineitem ORDER BY RANDOM()",
        params![],
    )?;

    // Calculate the total number of rows in the lineitem table
    let total_rows: i64 =
        conn.query_row("SELECT COUNT(*) FROM lineitem", params![], |row| row.get(0))?;
    let sample_size = (sample_fraction * total_rows as f64).round() as i64;

    // Populate s1_sample with a subset of rows based on the sample fraction
    conn.execute(
        "INSERT INTO s1_sample
         SELECT * FROM lineitem
         WHERE rowid IN (SELECT rowid FROM ids LIMIT ?)",
        params![sample_size],
    )?;
    // println!("s1_sample table created with sampled data.");

    // Create s2_sample by joining s1_sample with the orders table on the l_orderkey column
    conn.execute(
        "CREATE TABLE IF NOT EXISTS s2_sample AS
         SELECT s1.*, orders.*
         FROM s1_sample AS s1
         JOIN orders ON s1.l_orderkey = orders.o_orderkey",
        params![],
    )?;
    // println!("s2_sample table created with joined data.");

    // Create s3_sample by joining s2_sample with the customer table on the o_custkey column
    conn.execute(
        "CREATE TABLE IF NOT EXISTS s3_sample AS
         SELECT s2.*, customer.*
         FROM s2_sample AS s2
         JOIN customer ON s2.o_custkey = customer.c_custkey",
        params![],
    )?;
    // println!("s3_sample table created with joined data.");

    // Create s4_sample by joining s3_sample with the nation table on the c_nationkey column
    conn.execute(
        "CREATE TABLE IF NOT EXISTS s4_sample AS
         SELECT s3.*, nation.*
         FROM s3_sample AS s3
         JOIN nation ON s3.c_nationkey = nation.n_nationkey",
        params![],
    )?;
    // println!("s4_sample table created with joined data.");

    // Create s5_sample by joining s4_sample with the region table on the n_regionkey column
    conn.execute(
        "CREATE TABLE IF NOT EXISTS s5_sample AS
         SELECT s4.*, region.*
         FROM s4_sample AS s4
         JOIN region ON s4.n_regionkey = region.r_regionkey",
        params![],
    )?;
    // println!("s5_sample table created with joined data.");

    Ok(())
}
