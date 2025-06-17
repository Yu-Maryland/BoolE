import os
import glob
import subprocess

def run_abc_command():
    folder_path = '../case'

    # Iterate exponentially: 4, 8, 16, 32
    n = 4
    while n <= 32:
        aig_file = f'{folder_path}/{n}.aig'
        
        # Skip if file doesn't exist
        if not os.path.exists(aig_file):
            n *= 2
            continue
            
        filename = f'{n}'
        
        # Define the command and arguments
        command = 'abc'
        args = f'-c "read {folder_path}/{filename}.aig; st; &get; &write {folder_path}/{filename}_gia.aig"'

        # Run the command and redirect the output to log.txt
        log_file_name = 'log.txt'
        with open(log_file_name, 'a') as log_file:
            subprocess.run(f'{command} {args}', shell=True, stdout=log_file, stderr=subprocess.STDOUT)
        
        n *= 2

# Call the function to run the abc command
run_abc_command()
