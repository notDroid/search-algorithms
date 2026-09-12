### Unit Tests:
```bash
cargo test
```

### File Tests:
```bash
# all
cargo test --release -- --ignored correctness

# target
cargo test test_<algorithm>_<test_file> --release -- --ignored correctness

# example
cargo test test_negamax_pruned_l3_r1 --release -- --ignored correctness
```

### Metrics:
```bash
# all
cargo insta test --review --release -- --ignored metrics
```

### Benchmarks:
```bash
# all
cargo bench --verbose

# target
cargo bench <algorithm>_<test_file> --verbose

# example
cargo bench negamax_pruned_L3_R1 --verbose
```

