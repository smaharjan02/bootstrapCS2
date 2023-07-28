import os
import argparse
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
import csv
import numpy as np


def process_data_and_save(input_file):
    hit = 0
    hitlist = []
    data = []
    Total = 0
    Bootstrap_time = 0

    # Open the data logs from the experiment
    with open(input_file, "r") as file:
        reader = csv.reader(file)
        for row in reader:
            data.append(row)
        chunks = [data[x : x + 10] for x in range(1, len(data), 10)]
        # calculation of necessary fields for drawing the graphs
        for a in range(0, 10):
            hitlist.append([])
            for b in range(0, 10):
                if chunks[a][b][9] == "1":
                    hit += 1
                Total += float(chunks[a][b][8])
                Bootstrap_time += float(chunks[a][b][7])
            hit = hit * 100 / 10
            hitlist[a].append(int(chunks[a][b][1]))
            hitlist[a].append(hit)
            hitlist[a].append(round(Total, 2))
            hitlist[a].append(round(Bootstrap_time, 2))
            hit = 0
            Total = 0
            Bootstrap_time = 0

    # Get the folder name from the input file path
    folder_name = os.path.basename(os.path.dirname(input_file))
    # Get the parent directory of the input folder
    parent_dir = os.path.dirname(os.path.abspath(os.path.dirname(input_file)))
    # Create the new folder path to save the CSV files inside graph_experiments
    new_folder_path = os.path.join(
        parent_dir, "graph_experiments", "graph_" + folder_name
    )
    os.makedirs(new_folder_path, exist_ok=True)
    # Generate outputfile
    output_csv_file = os.path.join(
        new_folder_path,
        "graphlogs_{}.csv".format(os.path.splitext(os.path.basename(input_file))[0]),
    )

    # put the result in to csv file
    fields = ["Query Number", "Hit Percentage", "Total Time", "Bootstrap Time"]
    with open(output_csv_file, "w+", newline="") as file:
        writer = csv.writer(file, delimiter=",")
        writer.writerow(fields)
        for l in range(0, len(hitlist)):
            writer.writerow(hitlist[l])


# Create an argument parser
parser = argparse.ArgumentParser(description="Process CSV files and save graphs")

# Add the folder path argument
parser.add_argument(
    "-f",
    "--folder",
    type=str,
    help="Folder path containing CSV files",
    required=True,
)

# Parse the command-line arguments
args = parser.parse_args()

# Get the folder path from the command-line argument
folder_path = args.folder

# Get the parent directory of the input folder
parent_dir = os.path.dirname(os.path.abspath(folder_path))
# Create the "graph_experiments" folder to save processed CSV files
output_folder_path = os.path.join(parent_dir, "graph_experiments")
os.makedirs(output_folder_path, exist_ok=True)

csv_files = [file for file in os.listdir(folder_path) if file.endswith(".csv")]

# Process each CSV file in the folder
for csv_file in csv_files:
    file_path = os.path.join(folder_path, csv_file)
    process_data_and_save(file_path)
