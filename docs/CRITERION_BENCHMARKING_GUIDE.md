# Criterion.rs Benchmarking Guide for ProvChain

This guide explains how to use Rust's Criterion.rs library for statistical benchmarking of ProvChain, similar to Elixir's Benchee.

## Overview

Criterion.rs provides:
- **Statistical Analysis**: Confidence intervals, outlier detection, regression analysis
- **HTML Reports**: Beautiful charts and detailed performance analysis
- **Comparison**: Compare performance across different implementations
- **Reproducible Results**: Consistent benchmarking methodology

## Available Benchmarks

### 1. Consensus Benchmarks (`benches/consensus_benchmarks.rs`)
```bash
cargo bench --bench consensus_benchmarks
```

**Benchmarks:**
- **Block Creation**: Performance across different block sizes (1, 5, 10, 25, 50 blocks)
- **RDF Canonicalization**: Simple, complex, and supply chain RDF patterns
- **SPARQL Queries**: Query performance on populated blockchain
- **Blockchain Scaling**: Performance degradation with chain length
- **Consensus Comparison**: ProvChain PoA vs simulated PoW/PoS

### 2. RDF Canonicalization Benchmarks (`benches/rdf_canonicalization_benchmarks.rs`)
```bash
cargo bench --bench rdf_canonicalization_benchmarks
```

**Benchmarks:**
- **Complexity Levels**: Minimal, simple, moderate, complex, pathological graphs
- **Blank Node Patterns**: No blanks, simple, nested, circular, isomorphic
- **Scaling Tests**: Graph sizes from 10 to 200 nodes
- **Supply Chain Patterns**: Linear, branched, merged, complex provenance

### 3. Blockchain Performance Benchmarks (`benches/blockchain_performance_benchmarks.rs`)
```bash
cargo bench --bench blockchain_performance_benchmarks
```

**Benchmarks:**
- **Throughput**: Transaction batch processing (1-100 transactions)
- **Query Scaling**: SPARQL performance on growing chains (10-500 blocks)
- **Memory Efficiency**: Different RDF data types and complexity
- **Validation Performance**: Chain validation across different lengths
- **Concurrent Operations**: Mixed read/write operations simulation
- **Hash Computation**: Performance with different data sizes

## Running Benchmarks

### Basic Usage

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suite
cargo bench --bench consensus_benchmarks

# Run specific benchmark function
cargo bench --bench consensus_benchmarks bench_block_creation

# Run with specific pattern
cargo bench "block_creation"
```

### Advanced Options

```bash
# Generate HTML reports (enabled by default)
cargo bench --bench consensus_benchmarks -- --output-format html

# Run for longer measurement time
cargo bench --bench consensus_benchmarks -- --measurement-time 30

# Save baseline for comparison
cargo bench --bench consensus_benchmarks -- --save-baseline main

# Compare against baseline
cargo bench --bench consensus_benchmarks -- --baseline main

# Run with specific sample size
cargo bench --bench consensus_benchmarks -- --sample-size 1000
```

## Understanding Results

### Sample Output
```
block_creation/provchain_poa/1
                        time:   [2.1234 ms 2.1456 ms 2.1678 ms]
                        thrpt:  [461.23 elem/s 465.89 elem/s 470.55 elem/s]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
```

**Interpretation:**
- **time**: [lower_bound mean upper_bound] with 95% confidence interval
- **thrpt**: Throughput (elements per second)
- **outliers**: Statistical outliers detected and excluded

### HTML Reports

Criterion generates detailed HTML reports in `target/criterion/`:
- **Performance plots**: Time series and distribution charts
- **Regression analysis**: Performance trends over time
- **Comparison charts**: Side-by-side performance comparisons
- **Statistical details**: Confidence intervals, standard deviation

## Benchmark Configuration

### Custom Configuration

Create `benches/criterion_config.rs`:
```rust
use criterion::{Criterion, PlotConfiguration, AxisScale};

pub fn configure_criterion() -> Criterion {
    Criterion::default()
        .measurement_time(std::time::Duration::from_secs(10))
        .sample_size(100)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.02)
        .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic))
}
```

### Environment Variables

```bash
# Set measurement time
export CRITERION_MEASUREMENT_TIME=15

# Set sample size
export CRITERION_SAMPLE_SIZE=200

# Enable debug output
export CRITERION_DEBUG=1
```

## Comparing with Academic Standards

### Statistical Rigor
- **Confidence Intervals**: 95% confidence by default
- **Outlier Detection**: Automatic outlier removal
- **Multiple Runs**: Statistical significance testing
- **Regression Analysis**: Performance trend detection

### Publication-Ready Results
```bash
# Generate comprehensive report
cargo bench --bench consensus_benchmarks -- --output-format html csv

# Export data for analysis
cargo bench --bench consensus_benchmarks -- --output-format csv > results.csv
```

## Integration with CI/CD

### GitHub Actions Example
```yaml
name: Performance Benchmarks
on: [push, pull_request]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Run benchmarks
      run: cargo bench --bench consensus_benchmarks
    - name: Upload results
      uses: actions/upload-artifact@v2
      with:
        name: benchmark-results
        path: target/criterion/
```

## Performance Baselines

### Establishing Baselines
```bash
# Save current performance as baseline
cargo bench --bench consensus_benchmarks -- --save-baseline v1.0

# Compare new changes against baseline
cargo bench --bench consensus_benchmarks -- --baseline v1.0
```

### Regression Detection
```bash
# Fail if performance regresses by more than 10%
cargo bench --bench consensus_benchmarks -- --baseline v1.0 --threshold 0.1
```

## Academic Benchmarking Best Practices

### 1. Reproducible Environment
```bash
# Set CPU governor for consistent results
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Disable CPU frequency scaling
sudo cpupower frequency-set --governor performance
```

### 2. Multiple Runs
```bash
# Run benchmarks multiple times for statistical significance
for i in {1..5}; do
    cargo bench --bench consensus_benchmarks -- --save-baseline run_$i
done
```

### 3. System Monitoring
```bash
# Monitor system resources during benchmarks
htop &
cargo bench --bench consensus_benchmarks
```

## Benchmark Data Analysis

### Exporting Results
```rust
// Custom benchmark with data export
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn custom_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("custom");
    
    // Configure for academic rigor
    group.sample_size(1000);
    group.measurement_time(std::time::Duration::from_secs(30));
    
    // Your benchmark code here
    
    group.finish();
}
```

### Statistical Analysis
```python
# Python script for additional analysis
import pandas as pd
import matplotlib.pyplot as plt

# Load Criterion CSV output
df = pd.read_csv('target/criterion/consensus_benchmarks/report/index.csv')

# Generate publication-ready plots
plt.figure(figsize=(10, 6))
plt.plot(df['input_size'], df['mean_time'])
plt.xlabel('Input Size')
plt.ylabel('Execution Time (ms)')
plt.title('ProvChain Consensus Performance Scaling')
plt.savefig('consensus_scaling.pdf', dpi=300, bbox_inches='tight')
```

## Troubleshooting

### Common Issues

1. **Inconsistent Results**
   ```bash
   # Ensure stable system state
   sudo systemctl stop unnecessary-services
   cargo bench --bench consensus_benchmarks
   ```

2. **Memory Issues**
   ```bash
   # Reduce sample size for memory-intensive benchmarks
   cargo bench --bench rdf_canonicalization_benchmarks -- --sample-size 50
   ```

3. **Long Running Times**
   ```bash
   # Reduce measurement time for quick feedback
   cargo bench --bench blockchain_performance_benchmarks -- --measurement-time 5
   ```

## Integration with Research

### Paper-Ready Results
1. **Export data**: Use CSV format for statistical analysis
2. **Generate plots**: HTML reports provide publication-quality charts
3. **Statistical significance**: Confidence intervals and p-values included
4. **Reproducibility**: Exact configuration and environment details

### Comparison Studies
```bash
# Compare ProvChain against other implementations
cargo bench --bench consensus_benchmarks -- --save-baseline provchain
# ... run competitor benchmarks ...
cargo bench --bench consensus_benchmarks -- --baseline competitor
```

This comprehensive benchmarking setup provides the statistical rigor and detailed analysis needed for academic publication while being similar to Elixir's Benchee in terms of ease of use and detailed reporting.
