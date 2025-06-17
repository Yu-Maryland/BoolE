#!/bin/bash

# Display menu and get user choice
echo "=== BoolE Benchmark Generator ==="
echo "Please select a benchmark type to generate:"
echo "1) CSA multipliers"
echo "2) CSA mapped multipliers"
echo "3) Booth multipliers"
echo "4) Booth mapped multipliers"
echo "5) dch optimized multipliers"
read -p "Enter your choice (1-5): " choice

# Validate user input
if [[ ! $choice =~ ^[1-5]$ ]]; then
    echo "Invalid choice. Please run the script again and select a number from 1 to 5."
    exit 1
fi

# Create appropriate directory based on user choice
case $choice in
    1) mkdir -p csa
       echo "Generating CSA multipliers..."
       benchmark_type="csa"
       ;;
    2) mkdir -p booth
       echo "Generating Booth multipliers..."
       benchmark_type="booth"
       ;;
    3) mkdir -p csa_map
       echo "Generating CSA mapped multipliers..."
       benchmark_type="csa_map"
       ;;
    4) mkdir -p booth_map
       echo "Generating Booth mapped multipliers..."
       benchmark_type="booth_map"
       ;;
    5) mkdir -p dch
       echo "Generating dch optimized multipliers..."
       benchmark_type="dch"
       ;;
esac

# Path to ABC binary
ABC_PATH="../abc/abc"

# Ask for size range
start_size=4
end_size=128
step_size=4

# Generate benchmarks for the chosen type
for N in $(seq $start_size $step_size $end_size); do
    echo "Generating benchmark for N=$N..."
    
    case $choice in
        1) # CSA multiplier
           temp_script="gen -N $N -m csa/mul${N}.blif; st; write csa/mul${N}.aig"
           $ABC_PATH -c "$temp_script"
           ;;
        2) # Booth multiplier
           temp_script="gen -N $N -b booth/mul${N}_booth.blif; st; write booth/mul${N}_booth.aig"
           $ABC_PATH -c "$temp_script"
           ;;
        3) # CSA mapped multiplier
           temp_script="gen -N $N -m csa_map/mul${N}.blif; read 7nm.genlib; map; st; write csa_map/mul${N}_map.blif; write csa_map/mul${N}_map.aig"
           $ABC_PATH -c "$temp_script"
           # Remove the intermediate BLIF file
           rm -f csa_map/mul${N}.blif
           ;;
        4) # Booth mapped multiplier
           temp_script="gen -N $N -b booth_map/mul${N}_booth.blif; read 7nm.genlib; map; st; write booth_map/mul${N}_booth_map.blif; write booth_map/mul${N}_booth_map.aig"
           $ABC_PATH -c "$temp_script"
           # Remove the intermediate BLIF file
           rm -f booth_map/mul${N}_booth.blif
           ;;
        5) # dch optimized multiplier
           temp_script="gen -N $N -m dch/mul${N}.blif; st; dch -v; write dch/mul${N}_dch.blif; write dch/mul${N}_dch.aig"
           $ABC_PATH -c "$temp_script"
           # Remove the intermediate BLIF file
           rm -f dch/mul${N}.blif
           ;;
    esac
done

rm -f abc.history
echo "Benchmark generation completed!"
echo "Files are saved in the $benchmark_type directory."