import subprocess
import time

commands = [
    "python3 driver.py -df tpch_10g.db -qr query_third_join.sql -sr 0.5 -ls 10 -bs 200",
    "python3 driver.py -df tpch_10g.db -qr query_third_join.sql -sr 0.5 -ls 10 -bs 2000",
    "python3 driver.py -df tpch_10g.db -qr query_third_join.sql -sr 0.1 -ls 10 -bs 200",
    "python3 driver.py -df tpch_10g.db -qr query_third_join.sql -sr 0.1 -ls 10 -bs 2000",
    "python3 driver.py -df tpch_10g.db -qr query_fourth_join.sql -sr 1 -ls 10 -bs 200",
    "python3 driver.py -df tpch_10g.db -qr query_fourth_join.sql -sr 1 -ls 10 -bs 2000",
    "python3 driver.py -df tpch_10g.db -qr query_fourth_join.sql -sr 0.5 -ls 10 -bs 200",
    "python3 driver.py -df tpch_10g.db -qr query_fourth_join.sql -sr 0.5 -ls 10 -bs 2000",
    "python3 driver.py -df tpch_10g.db -qr query_fourth_join.sql -sr 0.1 -ls 10 -bs 200",
    "python3 driver.py -df tpch_10g.db -qr query_fourth_join.sql -sr 0.1 -ls 10 -bs 2000",
]

for cmd in commands:
    subprocess.run(cmd, shell=True)
    time.sleep(60)  # Sleep for 60 seconds (1 minute)
