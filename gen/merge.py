def read_rules_from_file(filepath):
    try:
        with open(filepath, 'r') as f:
            return set(line.strip() for line in f if line.strip())
    except FileNotFoundError:
        print(f"Warning: File {filepath} not found")
        return set()

def merge_rules(prefix):
    # Read rules from old and new files
    old_rules = read_rules_from_file(f'../results/{prefix}_rules_old.txt')
    new_rules = read_rules_from_file(f'../results/{prefix}_rules_new.txt')
    
    # Merge rules
    merged_rules = old_rules.union(new_rules)
    
    # Write merged rules to output file
    with open(f'../results/{prefix}_rules.txt', 'w') as f:
        for rule in sorted(merged_rules):
            f.write(rule + '\n')
    
    print(f"Merged {prefix} rules:")
    print(f"Old rules: {len(old_rules)}")
    print(f"New rules: {len(new_rules)}")
    print(f"Merged unique rules: {len(merged_rules)}")

def main():
    # Process both maj and xor rules
    for prefix in ['maj', 'xor']:
        merge_rules(prefix)

if __name__ == "__main__":
    main()