import re
import hashlib
import itertools

def hash_expr(expr):
    hasher = hashlib.md5()
    hasher.update(str(expr).encode('utf-8'))
    return hasher.hexdigest()

def remove_duplicates(exprs):
    unique_exprs = set()
    result = []
    for expr in exprs:
        expr_hash = hash_expr(expr)
        if expr_hash not in unique_exprs:
            unique_exprs.add(expr_hash)
            result.append(expr)
    return result

def parse_benchmark(file_path):
    inputs = []
    outputs = []
    operations = []

    with open(file_path, 'r') as file:
        for line in file:
            line = line.strip()
            if line.startswith("INPUT"):
                inputs.append(re.search(r'INPUT\((\w+)\)', line).group(1))
            elif line.startswith("OUTPUT"):
                outputs.append(re.search(r'OUTPUT\((\w+)\)', line).group(1))
            else:
                if line:
                    operations.append(line)
    
    return inputs, outputs, operations

def generate_rewrite_rule(inputs, outputs, operations, four_digit_array, prefix):
    op_map = {}
    for op in operations:
        result, expr = op.split(" = ")
        result = result.strip()
        expr = expr.strip().replace("(", " ").replace(")", "")
        op_map[result] = expr
    
    output = outputs[0]
    expr = op_map[output]

    def replace_operators(expression):
        tokens = re.findall(r'\b\d+\b', expression)
        for token in tokens:
            if token in op_map:
                expression = re.sub(r'\b{}\b'.format(token), f"({op_map[token]})", expression)
        return expression

    previous_expr = None
    while expr != previous_expr:
        previous_expr = expr
        expr = replace_operators(expr)
    
    # Replace logical operators with lowercase and remove commas
    expr = expr.replace("AND", "and").replace("NOT", "not").replace(",", " ")

    # Replace input variables in the expression
    for i, input_var in enumerate(inputs):
        expr = re.sub(r'\b{}\b'.format(input_var), f"?op{i+1}", expr)

    # Remove any unnecessary spaces in the final rewrite rule
    expr = re.sub(r'\s+', ' ', expr).strip()
    
    expr = f"({expr})"

    replacements = {
        0: "?op1",
        1: "?op2",
        2: "?op3"
    }

    if prefix == "xor":
        output = "xor3 ?op1 ?op2 ?op3"
    else:
        output = "maj ?op1 ?op2 ?op3"

    # exprs = []
    # variables = sorted(set(re.findall(r'\?op\d+', expr)))
    # combinations = list(itertools.product([True, False], repeat=len(variables)))

    # for comb in combinations:
    #     expr_copy = expr
    #     for i, var in enumerate(variables):
    #         expr_copy = expr_copy.replace(var, f"(not {var})" if comb[i] else var)
    #     exprs.append(expr_copy)
    
    # return exprs

    for i in range(4):
        if four_digit_array[i] == 1 and i != 3:
            pattern = re.escape(replacements[i])  # this correctly escapes '?op1' to '\\?op1'
            output = re.sub(r'(?<!\w){}(?!\w)'.format(pattern), f"(not {replacements[i]})", output)

        elif four_digit_array[i] == 1 and i == 3:
            for j in range(3):
                pattern = re.escape(replacements[j])  # this correctly escapes '?op1' to '\\?op1'
                output = re.sub(r'(?<!\w){}(?!\w)'.format(pattern), f"(not {replacements[j]})", output)

    return f'rewrite!("CUSTOM_RULE"; "{expr}" => "({output})"),'


def parse_results(file_path):
    results = []
    
    with open(file_path, 'r') as file:
        for line in file:
            if 'and' in line:
                parts = line.strip().split(' and ')
                path = parts[0]
                filename = parts[1]
                number = filename.split('/')[-1].split('.')[0]
                four_digit_array = [int(digit) for digit in number.zfill(4)]
                results.append((path, four_digit_array))
    
    return results

def main():
    prefixs = ["maj", "xor"]
    # rewrite rules
    for prefix in prefixs:
        file_path = f'../results/merged_{prefix}_results.txt'  # Path to the benchmark file
        results = parse_results(file_path)
        rules = []

        for path, four_digit_array in results:
            inputs, outputs, operations = parse_benchmark(path)
            # print(inputs, outputs, operations)
            rewrite_rule = generate_rewrite_rule(inputs, outputs, operations, four_digit_array, prefix)
            rules.append(rewrite_rule)
            # exprs = generate_rewrite_rule(inputs, outputs, operations, four_digit_array, prefix)
            # for expr in exprs:
            #     rules.append(f'rewrite!("CUSTOM_RULE"; "{expr}" => "({prefix} ?op1 ?op2 ?op3)"),')
            #     rules.append(f'rewrite!("CUSTOM_RULE"; "(not {expr})" => "({prefix} ?op1 ?op2 ?op3)"),')
            #     if prefix == "xor":
            #         rules.append(f'rewrite!("CUSTOM_RULE"; "{expr}" => "({prefix}3 ?op1 ?op2 ?op3)"),')
            #         rules.append(f'rewrite!("CUSTOM_RULE"; "(not {expr})" => "({prefix}3 ?op1 ?op2 ?op3)"),')

        unique_rules = remove_duplicates(rules)
        with open(f'../results/{prefix}_rules.txt', 'w') as output_file:
            for rule in unique_rules: 
                    output_file.write(rule + '\n')
    print("Done!")

if __name__ == "__main__":
    main()