# Implementing Search Algorithms
I'm practicing rust by implementing search algorithms for games like connect4 or chess.
In particular I implement negamax and alpha beta pruning with common variations.

I test, benchmark, collect metrics, and profile the algorithms against a connect4 implementation.

I'm using [pascal pons connect4 ai guide](http://blog.gamesolver.org/) (originally written in C++) as reference. The inspiration for doing this project is from 
[Sebastian Lague's Chess AI video](https://www.youtube.com/watch?v=U4ogK0MIzqk).

## Development Commands

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

## Todo
- [x] Add basic move ordering
- [x] Add transposition table for lower bound case
- [x] Use custom hashing algorithm for transposition table
- [ ] Add iterative deepening
- [ ] Anticipate losing moves & have better move ordering
- [ ] Implement a more space efficient transposition table
- [ ] Modify the transposition table for general case
- [ ] Add another game?
