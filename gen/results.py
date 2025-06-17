import os

def merge_results(output_dir, result_type):
    merged_results = []
    for folder in os.listdir(output_dir):
        if not folder.endswith('_gia'):
            result_file_path = os.path.join(output_dir, folder, f"{result_type}_results.txt")
            if os.path.isfile(result_file_path):
                with open(result_file_path, 'r') as result_file:
                    merged_results.extend(result_file.readlines())
    
    return merged_results

def write_merged_results(result_dir, result_type, merged_results):
    merged_file_path = os.path.join(result_dir, f"merged_{result_type}_results.txt")
    with open(merged_file_path, 'w') as merged_file:
        merged_file.writelines(merged_results)

def main():
    output_dir = '../case'
    result_dir = '../results'
    
    # Merge maj_results.txt
    maj_results = merge_results(output_dir, 'maj')
    write_merged_results(result_dir, 'maj', maj_results)
    
    
    # Merge xor_results.txt
    xor_results = merge_results(output_dir, 'xor')
    write_merged_results(result_dir, 'xor', xor_results)

if __name__ == "__main__":
    main()