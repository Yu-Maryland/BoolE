import os
import re
import glob
import subprocess
import time

def run_abc_command(file_name, log_file):
    # Define the command and arguments
    command = 'abc'
    args = f'-c "read ../case/{file_name}.aig;st;&get;&show -a"'

    # Run the command and redirect the output to log.txt
    subprocess.run(f'{command} {args}', shell=True, stdout=log_file, stderr=subprocess.STDOUT)

# which is used to get the groups of the adder structure
def Get_groups(number_string, file_name, num, log_file):
    # Define the list
    numbers = number_string.split()
    numbers= [int(num) for num in numbers]

    # Initialize an empty list to store the groups
    groups = []

    # Iterate over the list in steps of six
    for i in range(0, len(numbers), 6):
        # Slice the list and append the slice to the groups list
        group = numbers[i:i+6]
        if len(group) >= 3 and group[2] == 0:
            continue
        groups.append(numbers[i:i+6])

    log_file.write(f"{file_name}: FA:{len(groups)}; HA:{num//6 - len(groups)}\n")
    return groups

def Get_data(file_name):
    # Initialize an empty list to store the data
    data = []

    # Open and read the file
    with open(f'../case/{file_name}_gia.aig', 'r') as file:
        lines = file.readlines()

    # Flag to start recording
    start_recording = False

    # Iterate over each line
    for line in lines:
        # Split the line into parts
        parts = line.split()
        
        # Check if the line contains exactly 3 parts and all parts are digits
        if len(parts) == 3 and all(part.isdigit() for part in parts):
            start_recording = True
        
        # If start_recording is True, append the line to data
        if start_recording:
            # Check if the line contains only digits
            if all(part.isdigit() for part in parts):
                data.append(line.strip())
            else:
                break  # Stop recording if the line contains non-digit characters

    return data

def process_number(file, number, data, inputs, logic_lines):
    for line in data:
        parts = line.split()
        if parts[0] == str(number):
            logic_lines.append(f"{parts[0]} = AND({parts[1]}, {parts[2]})\n")
            # Check if the second or third number is odd
            if int(parts[1]) % 2 != 0:
                logic_lines.append(f"{parts[1]} = NOT ({int(parts[1]) - 1})\n")
                if int(parts[1]) - 1 not in inputs:
                    process_number(file, int(parts[1]) - 1, data, inputs, logic_lines)
            else:
                if int(parts[1]) not in inputs:
                    process_number(file, int(parts[1]), data, inputs, logic_lines)
            if int(parts[2]) % 2 != 0:
                logic_lines.append(f"{parts[2]} = NOT ({int(parts[2]) - 1})\n")
                if int(parts[2]) - 1 not in inputs:
                    process_number(file, int(parts[2]) - 1, data, inputs, logic_lines)
            else:
                if int(parts[2]) not in inputs:
                    process_number(file, int(parts[2]), data, inputs, logic_lines)
            break

def remove_duplicates_lines(lines):
    seen = set()
    unique_lines = []
    for line in lines:
        if line not in seen:
            unique_lines.append(line)
            seen.add(line)
    return unique_lines

# Add this helper function before write_xor_to_bench
def replace_whole_number(line, old_num, new_var):
    # Pattern to match whole numbers (not parts of larger numbers)
    pattern = fr'\b{old_num}\b'
    return re.sub(pattern, new_var, line)

def write_xor_to_bench(groups, data, file_name):
    # Create a directory
    directory = f"../case/{file_name}_gia"
    if not os.path.exists(directory):
        os.makedirs(directory)

    for i, group in enumerate(groups):
        filename = os.path.join(directory, f"xor_file_{i+1}.bench")
        with open(filename, 'w') as file:
            xor = group[3] * 2
            inputs = {group[0] * 2, group[1] * 2, group[2] * 2}
            logic_lines = []
            process_number(file, xor, data, inputs, logic_lines)
            # Write INPUT and OUTPUT lines
            file.write(f"INPUT(a)\n")
            file.write(f"INPUT(b)\n")
            file.write(f"INPUT(c)\n")
            file.write(f"OUTPUT(d)\n")
            file.write("\n")

            # Write logic lines in reverse order
            replaced_logic_lines = []
            for line in reversed(logic_lines):
                line = replace_whole_number(line, str(group[0] * 2), "a")
                line = replace_whole_number(line, str(group[1] * 2), "b")
                line = replace_whole_number(line, str(group[2] * 2), "c")
                line = replace_whole_number(line, str(xor), "d")
                replaced_logic_lines.append(line)

            unique_logic_lines = remove_duplicates_lines(replaced_logic_lines)

            # Write unique logic lines
            for line in unique_logic_lines:
                file.write(line)
                
def write_maj_to_bench(groups, data, file_name):
    # Create a directory
    directory = f"../case/{file_name}_gia"
    if not os.path.exists(directory):
        os.makedirs(directory)

    for i, group in enumerate(groups):
        filename = os.path.join(directory, f"maj_file_{i+1}.bench")
        with open(filename, 'w') as file:
            maj = group[4] * 2
            inputs = {group[0] * 2, group[1] * 2, group[2] * 2}
            logic_lines = []
            process_number(file, maj, data, inputs, logic_lines)
            # Write INPUT and OUTPUT lines
            file.write(f"INPUT(a)\n")
            file.write(f"INPUT(b)\n")
            file.write(f"INPUT(c)\n")
            file.write(f"OUTPUT(d)\n")
            file.write("\n")

            replaced_logic_lines = []
            for line in reversed(logic_lines):
                line = replace_whole_number(line, str(group[0] * 2), "a")
                line = replace_whole_number(line, str(group[1] * 2), "b")
                line = replace_whole_number(line, str(group[2] * 2), "c")
                line = replace_whole_number(line, str(maj), "d")
                replaced_logic_lines.append(line)

            unique_logic_lines = remove_duplicates_lines(replaced_logic_lines)

            # Write unique logic lines
            for line in unique_logic_lines:
                file.write(line)

def compare_files(file_name, prefix, log_file):
    # Set the directories
    directory = f"../case/{file_name}_gia"
    compare_directory = f"./struc_{prefix}"
    output_file = f"../case/{file_name}/{prefix}_results.txt"
    
    # Create the directory if it does not exist
    if not os.path.exists(f"../case/{file_name}/"):
        os.makedirs(f"../case/{file_name}/")

    # Open the output file in write mode
    with open(output_file, 'w') as out_file:
        # Loop through each file in the directory that starts with "prefix"
        for file in os.listdir(directory):
            if file.startswith(f"{prefix}"):
                file_path = os.path.join(directory, file)
                # Check if the file exists
                if not os.path.isfile(file_path):
                    log_file.write(f"File {file_path} not found! Skipping...\n")
                    continue

                # Loop through each file in the compare directory
                for compare_file in os.listdir(compare_directory):
                    compare_file_path = os.path.join(compare_directory, compare_file)
                    # Check if the compare file exists
                    if not os.path.isfile(compare_file_path):
                        log_file.write(f"File {compare_file_path} not found! Skipping...\n")
                        continue

                    # Run the ABC command for the current file and compare file
                    result = subprocess.run(
                        ["abc", "-c", f"cec {file_path} {compare_file_path}"],
                        capture_output=True,
                        text=True
                    )

                    # Check if the output contains "Networks are equivalent"
                    if "Networks are equivalent" in result.stdout:
                        out_file.write(f"{file_path} and {compare_file_path}\n")

def extract_maj_digits(filename, file_type):
    with open(filename, 'r') as file:
        lines = file.readlines()
        digits = {}
        for line in lines:
            # Use regular expressions to match 'file_type_x' and the four-digit number after it
            match = re.search(rf'({file_type}_(\d+)).*?/(\d{{4}})\.bench', line)
            if match:
                file_number = match.group(2)  # Extract the file number
                digits[file_number] = match.group(3)  # Extract the four-digit number
        return digits

def extract_xor_digits(filename, file_type):
    with open(filename, 'r') as file:
        lines = file.readlines()
        digits = {}
        for line in lines:
            # Use regular expressions to match 'file_type_x' and the four-digit number after it
            match = re.search(rf'({file_type}_(\d+)).*?/(\d{{4}})\.bench', line)
            if match:
                file_number = match.group(2)  # Extract the file number
                if file_number not in digits:
                    digits[file_number] = []
                digits[file_number].append(match.group(3))  # Extract the four-digit number
        return digits

def main():    
    # Define the folder path containing the .bench files
    folder_path = '../case'

    # Iterate exponentially: 4, 8, 16, 32
    n = 4
    while n <= 32:
        aig_file = os.path.join(folder_path, f'{n}.aig')
        
        # Skip if file doesn't exist
        if not os.path.exists(aig_file):
            continue
            
        file_name = f'{n}'
        
        # Ensure the log file directory exists
        log_file_path = f"../case/{file_name}/log.txt"
        os.makedirs(os.path.dirname(log_file_path), exist_ok=True)

        with open(log_file_path, 'a') as log_file:
            start_time = time.time()
            run_abc_command(file_name, log_file)
            end_time = time.time()
            elapsed_time = (end_time - start_time) * 1000
        
        with open(log_file_path, 'r', encoding='utf-8') as log_file:
            content = log_file.read()
            content = content.replace("sh: gv: command not found\n", "")

        with open(log_file_path, 'w') as log_file:
            log_file.write(f"execution time: {elapsed_time:.2f} milliseconds\n")

            num_match = re.search(r'Vector has (\d+) entries', content)
            num = int(num_match.group(1)) if num_match else None

            number_string_match = re.search(r'entries: \{ (.*?) \}', content)
            number_string = number_string_match.group(1) if number_string_match else None

            prefixs = {"maj", "xor"}

            groups = Get_groups(number_string, file_name, num, log_file)
            data = Get_data(file_name)

            write_xor_to_bench(groups, data, file_name)
            write_maj_to_bench(groups, data, file_name)
            
            for prefix in prefixs:
                compare_files(file_name, prefix, log_file)
            
            maj_digits = extract_maj_digits(f"../case/{file_name}/maj_results.txt", 'maj_file')
            xor_digits = extract_xor_digits(f"../case/{file_name}/xor_results.txt", 'xor_file')

            common_keys = set(maj_digits.keys()).intersection(xor_digits.keys())
            num = 0
            if common_keys:
                for key in common_keys:
                    if maj_digits[key] in xor_digits[key]:
                        num += 1

            log_file.write(f"number of exact full adders: {num}\n")
            
        n *= 2

if __name__ == "__main__":
    main()
