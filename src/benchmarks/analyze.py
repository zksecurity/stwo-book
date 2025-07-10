import os
import pandas as pd
import matplotlib.pyplot as plt
import itertools
import numpy as np
from jinja2 import Environment, FileSystemLoader
from tabulate import tabulate
import math
from matplotlib.ticker import FixedLocator

# Function to preprocess the dataframes
def preprocess_data(df, column_name):
    """Preprocess the dataframe by dropping NaN values and converting units."""
    df = df.dropna(subset=["n", column_name])
    df = df.sort_values(by="n").reset_index(drop=True)

    # Convert units if applicable
    if "prover time(ms)" in column_name:
        df[column_name] = df[column_name] / 1000  # Convert ms to s
    elif "proof size(bytes)" in column_name:
        df[column_name] = df[column_name] / 1000  # Convert bytes to KB

    return df[["n", column_name]]

# Function to combine benchmark data
def combine_benchmark(bench_tuple, column_name):
    """Combine benchmark data for a given column from multiple sources."""
    bench_name, is_precompile, is_builtin = bench_tuple
    file_paths = {
        "jolt": f'./benchmark-results/jolt-{bench_name}.csv',
        "sp1": f'./benchmark-results/sp1-{bench_name}.csv',
        "openvm": f'./benchmark-results/openvm-{bench_name}.csv',
        "r0": f'./benchmark-results/risczero-{bench_name}.csv',
        "stone": f'./benchmark-results/stone-{bench_name}.csv',
        "stwo": f'./benchmark-results/stwo-{bench_name}.csv',
    }

    if is_precompile:
        file_paths["sp1-precompile"] = f'./benchmark-results/sp1-{bench_name}-precompile.csv'
        file_paths["r0-precompile"] = f'./benchmark-results/risczero-{bench_name}-precompile.csv'
        file_paths["openvm-precompile"] = f'./benchmark-results/openvm-{bench_name}-precompile.csv'

    if is_builtin:
        file_paths["stone-precompile"] = f'./benchmark-results/stone-{bench_name}-builtin.csv'

    if bench_name == "ec":
        file_paths.pop("sp1-precompile", None)
        file_paths.pop("openvm-precompile", None)
        file_paths.pop("r0-precompile", None)

    if bench_name == "blake" or bench_name == "blake-chain":
        file_paths = {
            "sp1": f'./benchmark-results/sp1-{bench_name}.csv',
            "r0": f'./benchmark-results/risczero-{bench_name}.csv',
            "stwo-precompile": f'./benchmark-results/stwo-{bench_name}.csv',
        }

    
    combined_df = None

    for name, path in file_paths.items():
        df = preprocess_data(pd.read_csv(path), column_name)

        df.rename(columns={column_name: name}, inplace=True)

        if combined_df is None:
            combined_df = df
        else:
            combined_df = pd.merge(combined_df, df, on="n", how="outer")

    return combined_df


def plot_benchmark(df, title, y_label, bench_tuple, column_name):
    """Plot the benchmark data with fixed color and marker mapping per column."""
    os.makedirs("plots", exist_ok=True)
    bench_name, _, is_builtin = bench_tuple

    style_map = {
        'jolt': ('o', '#644172'),
        'r0': ('s', '#00FF00'),
        'sp1': ('D', '#FE11C5'),
        'stone': ('^', '#236B8E'),
        'stwo': ('v', '#EC5631'),
        'r0-precompile': ('*', '#699C52'),
        'sp1-precompile': ('P', '#DC75CD'),
        'stone-precompile': ('h', '#58C4DD'),
        'stwo-precompile': ('H', '#F2806B'),
        'openvm': ('X', '#505050'),
        'openvm-precompile': ('d', '#A0A0A0'),
    }

    plt.figure(figsize=(8, 6))

    for col in df.columns[1:]:
        if col in style_map:
            marker, color = style_map[col]
        else:
            marker, color = ('x', 'gray')

        non_zero = df[col] != 0
        plt.plot(df["n"][non_zero], df[col][non_zero], 
                    marker=marker, color=color, label=col, linestyle='-')

    plt.title(title)
    plt.yscale("log")
    plt.xscale("log")

    num_legend = len(df.columns) - 1
    ncol = math.ceil(num_legend / 2)

    plt.legend(
        loc='upper center',
        bbox_to_anchor=(0.5, -0.15),
        ncol=ncol,
        fontsize="small"
    )
    plt.subplots_adjust(bottom=0.2)
    plt.grid(True, which="both", linestyle="--", linewidth=0.5)

    # plt.xticks(df["n"], df["n"], rotation=45)

    ax = plt.gca()
    ax.set_xscale("log")  # Keep log scale
    ax.xaxis.set_major_locator(FixedLocator(df["n"]))  # Force only these ticks
    ax.set_xticklabels(df["n"], rotation=45)  # Label them
    ax.xaxis.set_minor_locator(FixedLocator([]))

    # Save the plot
    filename = f"./plots/{bench_name}_{title.replace(' ', '_').lower()}.png"
    plt.savefig(filename, dpi=300)
    plt.close()

    return filename


def get_data(bench_tuple):
    prover_time_df = combine_benchmark(bench_tuple, "prover time(ms)")
    verifier_time_df = combine_benchmark(bench_tuple, "verifier time(ms)")
    proof_size_df = combine_benchmark(bench_tuple, "proof size(bytes)")
    cycle_count_df = combine_benchmark(bench_tuple, "cycle count")
    cycle_count_df = cycle_count_df.drop(columns=['openvm', 'openvm-precompile'], errors='ignore')
    peak_memory_df = combine_benchmark(bench_tuple, "peak memory")
    return prover_time_df, verifier_time_df, proof_size_df, cycle_count_df, peak_memory_df

def get_tables(bench_tuple):
    data_dfs_tuple = get_data(bench_tuple)
    df_names = ["prover_time", "verifier_time", "proof_size", "cycle_count", "peak_memory"]
    tables = {}

    for i, df_original in enumerate(data_dfs_tuple):
        # Format all numbers to plain string (no scientific notation)
        for col in df_original.columns:
            if pd.api.types.is_numeric_dtype(df_original[col]):
                df_original[col] = df_original[col].map(
                    lambda x: f"{x:.0f}" if isinstance(x, (int, float)) and abs(x) >= 1e5 else str(x)
                )
            else:
                df_original[col] = df_original[col].astype(str)

        # Replace NaN values
        df_processed = df_original.copy()
        for col in df_processed.columns:
            if "stone" in col.lower():
                df_processed[col] = df_processed[col].replace({"nan": "💾", "NaN": "💾", "": "💾"})
            else:
                df_processed[col] = df_processed[col].replace({"nan": "❌", "NaN": "❌", "": "❌"})
        
        # Convert DataFrame to table format without transposing
        table_data = df_processed.values.tolist()
        headers = df_processed.columns.tolist()

        # Create right alignment for all columns
        colalign = ["right"] * len(headers)
        table_markdown = tabulate(table_data, headers=headers, tablefmt="github", colalign=colalign)
        
        # Manually modify the markdown to right-align all columns
        lines = table_markdown.split('\n')
        if len(lines) >= 2:
            # Replace the alignment row (second line) to right-align all columns
            alignment_parts = lines[1].split('|')
            new_alignment = '|'
            for j, part in enumerate(alignment_parts[1:-1]):  # Skip first and last empty parts
                new_alignment += '----:|'  # Right align all columns
            lines[1] = new_alignment
            table_markdown = '\n'.join(lines)
        
        tables[df_names[i]] = table_markdown

    return tables

def get_plots(bench_tuple):
    prover_time_df, verifier_time_df, proof_size_df, cycle_count_df, peak_memory_df = get_data(bench_tuple)
    prover_time_plot = plot_benchmark(prover_time_df, "Prover Time vs n", "Prover Time (s)", bench_tuple, "prover time(ms)")
    verifier_time_plot = plot_benchmark(verifier_time_df, "Verifier Time vs n", "Verifier Time (ms)", bench_tuple, "verifier time(ms)")
    proof_size_plot = plot_benchmark(proof_size_df, "Proof Size vs n", "Proof Size (KB)", bench_tuple, "proof size(bytes)")
    cycle_count_plot = plot_benchmark(cycle_count_df, "Cycle Count vs n", "Cycle Count", bench_tuple, "cycle count")
    peak_memory_plot = plot_benchmark(peak_memory_df, "Peak Memory vs n", "Peak Memory", bench_tuple, "peak memory")

    plots = {
        "prover_time_plot": prover_time_plot,
        "verifier_time_plot": verifier_time_plot,
        "proof_size_plot": proof_size_plot,
        "cycle_count_plot": cycle_count_plot,
        "peak_memory_plot": peak_memory_plot,
    }

    return plots

# Commit Info
commit_file = "./benchmark-results/latest_commit.txt"
with open(commit_file, "r") as file1:
    commit_hash = file1.readline().strip()

# Timestamp Info
time_file = "./benchmark-results/time_stamp.txt"
with open(time_file, "r") as file1:
    time = file1.readline().strip()


# OS Info
os_version_file = "./benchmark-results/os_version.txt"
os_version = "Unknown"
with open(os_version_file, "r") as file:
    for line in file:
        if line.startswith("PRETTY_NAME="):
            os_version = line.split("=", 1)[1].strip().strip('"')
            break

# CPU Info
cpuinfo_file = "./benchmark-results/cpuinfo.txt"
cpu_keys = [
    "Architecture",
    "CPU(s)",
    "Model name",
    "Thread(s) per core",
    "Core(s) per socket",
    "Socket(s)",
    "L3 cache"
]

cpu_info = {}

with open(cpuinfo_file, "r") as file:
    for line in file:
        for key in cpu_keys:
            if line.startswith(key):
                value = line.split(":", 1)[1].strip()
                cpu_info[key] = value

# Memory Info
meminfo_file = "./benchmark-results/meminfo.txt"
mem_keys = [
    "MemTotal",
    "MemFree",
    "MemAvailable",
]

mem_info = {}

with open(meminfo_file, "r") as file:
    for line in file:
        key_value = line.split(":", 1)
        if len(key_value) == 2:
            key, value = key_value[0].strip(), key_value[1].strip()
            if key in mem_keys:
                # Convert from KB to GB
                value_kb = value.replace(" kB", "").strip()
                try:
                    value_gb = float(value_kb) / (1024 * 1024)
                    mem_info[key] = f"{value_gb:.2f} GB"
                except ValueError:
                    mem_info[key] = value


# Fibonacci
fib_tuple = ("fib", False, False)
fib_tables = get_tables(fib_tuple)
fib_plots = get_plots(fib_tuple)
fib_data = {"tables": fib_tables, "plots": fib_plots}

# Sha2
sha2_tuple = ("sha2", True, False)
sha2_tables = get_tables(sha2_tuple)
sha2_plots = get_plots(sha2_tuple)
sha2_data = {"tables": sha2_tables, "plots": sha2_plots}

# Sha2-chain
sha2_chain_tuple = ("sha2-chain", True, False)
sha2_chain_tables = get_tables(sha2_chain_tuple)
sha2_chain_plots = get_plots(sha2_chain_tuple)
sha2_chain_data = {"tables": sha2_chain_tables, "plots": sha2_chain_plots}

# Sha3
sha3_tuple = ("sha3", True, True)
sha3_tables = get_tables(sha3_tuple)
sha3_plots = get_plots(sha3_tuple)
sha3_data = {"tables": sha3_tables, "plots": sha3_plots}

# Sha3-chain
sha3_chain_tuple = ("sha3-chain", True, True)
sha3_chain_tables = get_tables(sha3_chain_tuple)
sha3_chain_plots = get_plots(sha3_chain_tuple)
sha3_chain_data = {"tables": sha3_chain_tables, "plots": sha3_chain_plots}

# Mat Mul
mat_mul_tuple = ("mat-mul", False, False)
mat_mul_tables = get_tables(mat_mul_tuple)
mat_mul_plots = get_plots(mat_mul_tuple)
mat_mul_data = {"tables": mat_mul_tables, "plots": mat_mul_plots}

# ec
ec_tuple = ("ec", True, False)
ec_tables = get_tables(ec_tuple)
ec_plots = get_plots(ec_tuple)
ec_data = {"tables": ec_tables, "plots": ec_plots}

# Load template
env = Environment(loader=FileSystemLoader("."))
template = env.get_template("template.md.j2")

# Render Markdown
output_md = template.render(
    commit_hash=commit_hash,
    time=time,
    os_version=os_version,
    cpu_info=cpu_info,
    mem_info=mem_info,
    fib_data=fib_data,
    sha2_data=sha2_data,
    sha2_chain_data=sha2_chain_data,
    sha3_data=sha3_data,
    sha3_chain_data=sha3_chain_data,
    mat_mul_data=mat_mul_data,
    ec_data=ec_data,
)

# Save to index.md
with open("index.md", "w") as f:
    f.write(output_md)

print("index.md generated.")