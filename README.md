# bootstrapCS2

This is a implementation of Scalable Correlated Sampling for Join Query Estimations with bootstrap sampling

# run the code

Make sure to install sqlite and rust inorder to run the code

cd src

# Run Single time

cargo run -- -d tpch_100m.db -s 0.1 -b 1000

-d = name of the sqlite database
-s = sample ratio
-b bootstrap size or number

# Run multiple query multiple time

python3 driver.py -df data.db -qr query.sql -sr 0.5 -ls 10 -bs 100

-df = name of the sqlite database
-qr = file with 10 queries to run
-sr = sample ratio
-ls = loop size
-bs = bootstrap size or number
