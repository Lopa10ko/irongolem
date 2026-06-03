# irongolem

Graph Optimiser for Learning and Evolution of Models

Rust reimplementation of evolutionary graph optimization (spec derived from [GOLEM](https://github.com/aimclub/GOLEM)).


## Layout

```
Cargo.toml              # workspace root (run cargo here)
crates/
  irongolem/            # core library
  test-support/         # test fixtures & stub API
irongolem-tests/        # unit test package + JSON fixtures
```

## Deferred

See [`irongolem-tests/tests/unit/DEFERRED.md`](irongolem-tests/tests/unit/DEFERRED.md): integration tests, adaptive, tuning, visualisation.

## License

BSD-3-Clause — see [LICENSE](LICENSE).
