### Unit Tests:
```bash
cargo test
```

### File Tests:
```bash
# all
cargo test --release --test correctness -- --ignored --test-threads=<N>

# target
cargo test --release --test correctness -- --ignored <test_file>_<game_impl>::<algorithm>

# example
cargo test --release --test correctness -- --ignored l3_r1
```

### Metrics:
```bash
# all
cargo insta test --review --release --test metrics -- --ignored

# target
cargo insta test --review --release --test metrics -- --ignored <test_file>_<game_impl>::<algorithm>

# example
cargo insta test --review --release --test metrics -- --ignored l3_r1
```

### Benchmarks:
```bash
# all
cargo bench --verbose

# target
cargo bench <game_name>_<test_file>_<game_impl>/<algorithm> --verbose

# example
cargo bench negamax_pruned_L3_R1 --verbose
```

### Profiling with Samply:
```bash
samply record cargo bench <game_name>_<test_file>_<game_impl>/<algorithm> --verbose
```