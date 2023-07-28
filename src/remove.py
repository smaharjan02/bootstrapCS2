import os

# Specify the directory path
directory_path = "/home/smaharjan02/Desktop/CS2_bootstrap/bootstrapCS2/src/data_100m"

# List all files in the directory
files = os.listdir(directory_path)

# Iterate over each file
for filename in files:
    if filename.endswith(".tbl"):
        # Create the file path
        file_path = os.path.join(directory_path, filename)
        # Create a temporary file path for writing modified lines
        temp_file_path = file_path + ".tmp"

        # Read the file line by line and write modified lines to the temporary file
        with open(file_path, "r") as input_file, open(
            temp_file_path, "w"
        ) as output_file:
            for line in input_file:
                modified_line = line.rstrip("| \n") + "\n"
                output_file.write(modified_line)

        # Replace the original file with the temporary file
        os.replace(temp_file_path, file_path)
