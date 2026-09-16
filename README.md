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
cargo test --release --test correctness -- --ignored l3_r1_bitboard::pruned
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
cargo bench connect4_L3_R1_bitboard/negamax_pruned --verbose  
```

### Profiling with Samply:
```bash
samply record cargo bench <game_name>_<test_file>_<game_impl>/<algorithm> --verbose
```

### Todo
- [] Add basic move ordering
- [] Add transposition table
- [] Add iterative deepening
- 