# Dijkstra Algorithm Benchmarks

Comprehensive benchmarks for the Dijkstra's algorithm implementation in the grid module.

## Running Benchmarks

Run all benchmarks:
```bash
cargo bench --bench dijkstra
```

Run specific benchmark group:
```bash
cargo bench --bench dijkstra dijkstra_uniform
cargo bench --bench dijkstra dijkstra_obstacles
cargo bench --bench dijkstra dijkstra_path
cargo bench --bench dijkstra dijkstra_path_obstacles
cargo bench --bench dijkstra dijkstra_variable_costs
```

Test benchmarks (faster, no actual benchmarking):
```bash
cargo bench --bench dijkstra -- --test
```

## Benchmark Groups

### dijkstra_uniform
Tests the basic `dijkstra()` function on uniform cost grids (all cells cost 1).
- **Grid sizes**: 10×10, 25×25, 50×50, 100×100
- **Purpose**: Baseline performance on simple grids

### dijkstra_obstacles
Tests `dijkstra()` on grids with obstacles (impassable cells).
- **Grid sizes**: 10×10, 25×25, 50×50, 100×100
- **Obstacle pattern**: Regular pattern with ~20% obstacles
- **Purpose**: Performance when paths need to route around obstacles

### dijkstra_path
Tests the `dijkstra_path()` function which finds the complete path.
- **Grid sizes**: 10×10, 25×25, 50×50, 100×100
- **Path**: Corner to corner (0,0) → (n-1, n-1)
- **Purpose**: Performance of path reconstruction

### dijkstra_path_obstacles
Tests `dijkstra_path()` with obstacles.
- **Grid sizes**: 10×10, 25×25, 50×50, 100×100
- **Purpose**: Path finding around obstacles

### dijkstra_variable_costs
Tests `dijkstra()` with variable terrain costs (1-9).
- **Grid sizes**: 10×10, 25×25, 50×50, 100×100
- **Purpose**: Performance with different edge weights

## Expected Performance

Approximate performance on modern hardware:

| Grid Size | Uniform Grid | With Obstacles | Path Finding |
|-----------|-------------|----------------|--------------|
| 10×10     | ~18 µs      | ~14 µs         | ~24 µs       |
| 25×25     | ~124 µs     | ~106 µs        | ~164 µs      |
| 50×50     | ~520 µs     | ~442 µs        | ~691 µs      |
| 100×100   | ~2.2 ms     | ~1.9 ms        | ~2.9 ms      |

Note: With obstacles, the algorithm has fewer cells to visit, so it can be faster than uniform grids.

## Output

Criterion generates detailed reports in `target/criterion/`:
- HTML reports with graphs
- Statistical analysis
- Historical comparison

Open the reports:
```bash
open target/criterion/report/index.html
```
