# irongolem

Graph Optimiser for Learning and Evolution of Models

Rust reimplementation of evolutionary graph optimization (spec derived from [GOLEM](https://github.com/aimclub/GOLEM)).


## Layout

```
Cargo.toml              # workspace root (run cargo here)
crates/
  irongolem/            # GOLEM port (all framework logic)
  test-support/         # test fixtures + helpers
irongolem-tests/        # unit test package + JSON fixtures
examples/               # matched Python vs Rust custom-graph bench
```

## Experiments (Python GOLEM vs Rust)

See [`examples/README.md`](examples/README.md). Short version:

```bash
python3 -m venv .venv && source .venv/bin/activate
pip install -e GOLEM

python examples/bench_custom.py --trials 5 --generations 10 --pop-size 5 \
  --output examples/results/python_custom.json

cargo run -p irongolem-bench --release -- --trials 5 --generations 10 --pop-size 5 \
  --output examples/results/rust_custom.json

cargo run -p irongolem-bench --release -- --compare \
  examples/results/python_custom.json examples/results/rust_custom.json
```

This is a **matched toy problem** (same graphs, operators, generation budget). It does not run GOLEM's NetworkX tree-search examples.

## Deferred

See [`irongolem-tests/tests/unit/DEFERRED.md`](irongolem-tests/tests/unit/DEFERRED.md): integration tests, adaptive, tuning, visualisation.

## License

BSD-3-Clause — see [LICENSE](LICENSE).
