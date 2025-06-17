
# BoolE: Exact Symbolic Reasoning via Boolean Equality Saturation (DAC'25)

[![arXiv](https://img.shields.io/badge/arXiv-2504.05577-b31b1b.svg)](https://arxiv.org/abs/2504.05577)

BoolE is a research prototype for exact symbolic reasoning via Boolean equality saturation, as described in our [DAC'25 paper](https://arxiv.org/abs/2504.05577).

---

## 📄 Paper

If you use BoolE in your research, please cite our paper:

```bibtex
@article{yin2025boole,
  title={BoolE: Exact Symbolic Reasoning via Boolean Equality Saturation},
  author={Yin, Jiaqi and Song, Zhan and Chen, Chen and Hu, Qihao and Yu, Cunxi},
  journal={arXiv preprint arXiv:2504.05577},
  year={2025}
}
```

---

## 🚀 Quick Start

### 1. Clone the Repository (with Submodules)

BoolE depends on external submodules. Make sure to **recursively** clone:

```bash
git clone --recursive https://github.com/Yu-Maryland/BoolE.git
cd BoolE
```

If you already cloned without `--recursive`, initialize submodules with:

```bash
git submodule update --init --recursive
```

---

### 2. Install Required Rust Environment

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed (recommended: stable toolchain):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, ensure `cargo` and `rustc` are in your PATH:

```bash
rustc --version
cargo --version
```

---

### 3. Build the ABC Binary

BoolE requires the `abc` binary (in the `abc` submodule) for certain functionalities. Accelerate the build process by enabling parallelism with `make -j` (replace `<N>` with the number of cores you want to use):

```bash
cd abc
make -j$(nproc)
cd ..
```

After building, make sure the generated `abc` binary is in your PATH or reference its path as needed.

---

### 4. Generating Benchmarks

Navigate to the benchmark directory and make the script executable:

```bash
cd benchmark
chmod +x gen.sh
./gen.sh
```

The script supports generating five types of multiplier benchmarks:

1) CSA multipliers
2) CSA mapped multipliers
3) Booth multipliers
4) Booth mapped multipliers
5) dch optimized multipliers

You'll be prompted to select a benchmark type (1-5). The script will then generate multipliers from size 4 to 128 (in steps of 4) in the appropriate directory.

When choosing options 2 or 4, the script uses 7nm.genlib (a 7nm technology library) to map the generated circuits to standard cells, producing both .blif and .aig output files with the "_map" suffix.

---

### 5. Usage

Run BoolE in the root directory using Cargo with the desired features:

```bash
cargo run --features "faster_greedy_dag_fa_mt" --release -- <benchmark> <true/false>
```

- `<benchmark>`: Path to the benchmark aig file to analyze.
- `<true/false>`: The expected property (true/false). Setting this to `true` generates detailed logs, while `false` produces minimal output.

**Example:**

```bash
cargo run --features "faster_greedy_dag_fa_mt"  --release -- benchmark/dch/mul32_dch.aig true
```

After execution, BoolE generates output files in the `./output` directory, including:
- `mul32_dch_boole.aig` - The AIG (And-Inverter Graph) representation
- `mul32_dch_boole.blif` - The BLIF (Berkeley Logic Interchange Format) representation

---

### 6. Formal Verification with RevSCA-2.0

This repository includes [RevSCA-2.0](https://github.com/amahzoon/RevSCA-2.0.git) as a submodule for fast SCA-based formal verification of integer multipliers.

**Navigate to the RevSCA-2.0 directory:**
```bash
cd RevSCA-2.0
```

**Fast verification example (with BoolE):**
```bash
./revsca ../output/mul32_dch_boole.aig mul32_dch_boole.txt -u
```
This runs very quickly and completes formal verification in seconds.

**Slow verification example (without BoolE):**
```bash
./revsca ../benchmark/dch/mul32_dch_boole.aig mul32_dch_boole.txt -u
```
This version (using the original benchmark AIG file) may time out even after a day.


---

## 📧 Contact

For questions or feedback, please reach out to the authors listed in the [paper](https://arxiv.org/abs/2504.05577) or open an issue in this repository.

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## 🧩 Third-Party Components

BoolE integrates the following external components:

- **ABC**: System for Sequential Synthesis and Verification
  - Website: http://www.eecs.berkeley.edu/~alanmi/abc/
  - Licensed by The Regents of the University of California
  - The full license text is included in [LICENSE.md](LICENSE.md)

- **RevSCA-2.0**: SCA-based formal verification tool for integer multipliers
  - Website: http://www.sca-verification.org/revsca
  - Developed by Alireza Mahzoon, Daniel Große, and Rolf Drechsler
  - The full license text is included in [LICENSE.md](LICENSE.md)

---

Enjoy Boolean equality saturation with BoolE!