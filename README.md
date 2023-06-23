# bootstrapCS2

This is a implementation of Scalable Correlated Sampling for Join Query Estimations with bootstrap sampling

# run the code

cd src

# Run Single time

cargo run -- -d tpch_100m.db -s 0.1 -b 1000

-d = name of the sqlite database
-s = sample ratio
-b bootstrap size or number

# Run multiple query multiple time

python3 driver.py -df tpch_100m -sr 0.1 -ls 10 -bs 1000

-df = name of the sqlite database
-sr = sample ratio
-ls = loop size
-bs = bootstrap size or number
