# bootstrapCS2

This repository contains an implementation of Scalable Correlated Sampling for Join Query Estimations with bootstrap sampling.

## How to Run the Code

Before running the code, make sure you have the following dependencies installed:

- SQLite
- Rust

To run the code, follow these steps:

1. Open your terminal and navigate to the `src` directory of the repository.

2. To run a single time estimation, use the following command:

   ```
   cargo run -- -d tpch_100m.db -s 0.1 -b 1000
   ```

   Here's the explanation of the command-line arguments:

   - `-d`: Specifies the name of the SQLite database.
   - `-s`: Specifies the sample ratio.
   - `-b`: Specifies the bootstrap size or number.

3. To run multiple queries multiple times, use the `driver.py` script along with the following command:

   ```
   python3 driver.py -df data.db -qr query.sql -sr 0.5 -ls 10 -bs 100
   ```

   Here's the explanation of the command-line arguments:

   - `-df`: Specifies the name of the SQLite database.
   - `-qr`: Specifies the file with the 10 queries to run.
   - `-sr`: Specifies the sample ratio.
   - `-ls`: Specifies the loop size.
   - `-bs`: Specifies the bootstrap size or number.

## Example Usage

To run a single time estimation with a database named `tpch_100m.db`, a sample ratio of 0.1, and a bootstrap size of 1000, use the following command:

```
cargo run -- -d tpch_100m.db -s 0.1 -b 1000
```

To run multiple queries multiple times with a database named `data.db`, a sample ratio of 0.5, a loop size of 10, and a bootstrap size of 100, use the following command:

```
python3 driver.py -df data.db -qr query.sql -sr 0.5 -ls 10 -bs 100
```

Feel free to adjust the command-line arguments according to your requirements.

**Note:** Make sure to replace `tpch_100m.db`, `data.db`, and `query.sql` with the actual names of your database file and query file, respectively.

## Setting up SQLite Database For Experimentation

To set up the SQLite database using the TPC-H dataset, follow these steps:

1. Generate TPC-H Dataset:

   We use the TPC-H-skew repository, available at https://github.com/YSU-Data-Lab/TPC-H-Skew.

2. Generate Test Data:

   For instance, to create a 100MB test dataset, run the following command:
   
   ```
   ./dbgen -s 0.1
   ```
   
4. Organize Data:
   
   Place the generated data in a folder, e.g., data_100m.
   
   Move the data_100m folder to the src directory of your project.

6. Data Cleanup:

   Execute the remove.py script to remove any extra | characters at the end of the data.

8. Create SQLite Database:

   Use the following command to create the SQLite3 database, named tpch_100m.db, and populate the tables with the data from data_100m:
   
   ```
   sqlite3 tpch_100m.db < data_import.sql
   ```
   
Replace folder names and database filenames as needed for your project.

These steps ensure the setup of the SQLite database with TPC-H datasets. Feel free to reach out if you have any questions or need further clarification!

Enjoy using the bootstrapCS2 implementation!
