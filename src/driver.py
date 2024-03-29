import subprocess
import re
import csv
import os.path
from os import path
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("-df", "--data_file", help="data file", required=True)
parser.add_argument("-qr", "--query_file", help="query file", required=True)
parser.add_argument(
    "-sr", "--sample_ratio", type=float, help="sample ratio", required=True
)
parser.add_argument("-ls", "--loop_time", type=int, help="loop time", required=True)
parser.add_argument(
    "-bs", "--bootstrap_size", type=int, help="bootstrap size", required=True
)
args = parser.parse_args()
print(args)

data_file = args.data_file
query_file = args.query_file
sample_ratio = args.sample_ratio
loop_time = args.loop_time
bootstrap_size = args.bootstrap_size

# Extract the base name of the query file
query_base_name = os.path.splitext(os.path.basename(query_file))[0]

# Create the folder name based on the query file name
folder_name = f"experiment_results_{query_base_name}"

log_file = os.path.join(
    folder_name,
    f"log_{data_file}_sr{sample_ratio}_lt{loop_time}_bs{bootstrap_size}.csv",
)

f_results = []
execution_times = []
logs = []
total_time = 0
hitflag = []
count = 0
with open(query_file, "r") as file1:
    lines = file1.readlines()
    query_index = 0
    total_queries = 0
    for line in lines:
        if line.startswith("-"):
            continue
        total_queries += 1
    print("total queries: {}\n".format(total_queries))

    for line in lines:
        hit = 0
        if line.startswith("-"):
            continue
        count += 1
        print("exp query {} of {}".format(count, total_queries))
        with open("query.txt", "w+") as file2:
            file2.write(line)

        for x in range(loop_time):
            command = "cargo run -- -d {} -s {} -b {}".format(
                data_file, sample_ratio, bootstrap_size
            )
            cp = subprocess.run(command, shell=True, stdout=subprocess.PIPE)

            results = cp.stdout.decode("utf-8")
            results = re.findall(r"\d*\.\d+|\d+", results)
            f_results.append([float(i) for i in results])

        for i in range(0, len(f_results)):
            if f_results[i][4] < f_results[i][0] < f_results[i][5]:
                hit += 1
                hitflag.append(1)
            else:
                hitflag.append(0)

            execution_times.append(f_results[i][-1])
        print("number of hit: {} of {}".format(hit, loop_time))
        hitpercent = hit * 100 / loop_time
        print("hit percentage: {}".format(hitpercent))

        for j in range(len(execution_times)):
            total_time += execution_times[j]
        print("total_time: {:.2f}".format(total_time))

        for k in range(0, len(f_results)):
            logs.append([k])
            logs[k].append(count)
            logs[k].append(f_results[k][0])
            logs[k].append(f_results[k][1])
            logs[k].append(f_results[k][3])
            logs[k].append(f_results[k][4])
            logs[k].append(f_results[k][5])
            logs[k].append(f_results[k][2])
            logs[k].append(f_results[k][7])
            logs[k].append(hitflag[k])

        fields = [
            "Run number",
            "Query Number",
            "GT",
            "EST",
            "STD",
            "Lower_CI",
            "Upper_CI",
            "RNG_Time",
            "Total_Time",
            "Hit",
        ]
        if not path.exists(folder_name):
            os.makedirs(folder_name)

        if not path.exists(log_file):
            with open(log_file, "w", newline="") as file:
                writer = csv.writer(file, delimiter=",")
                writer.writerow(fields)

        with open(log_file, "a", newline="") as file:
            writer = csv.writer(file, delimiter=",")
            for l in range(0, len(logs)):
                writer.writerow(logs[l])
        f_results.clear()
        logs.clear()
        execution_times.clear()
        hitflag.clear()
