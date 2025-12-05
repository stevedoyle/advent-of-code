use aoc2025::grid;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn create_grid(size: usize) -> Vec<Vec<u32>> {
    (0..size).map(|_| (0..size).map(|_| 1).collect()).collect()
}

fn create_grid_with_obstacles(size: usize) -> Vec<Vec<u32>> {
    let mut grid = vec![vec![0; size]; size];

    // Add some obstacles in a pattern
    for i in (1..size).step_by(3) {
        for j in (1..size).step_by(2) {
            if i < size && j < size {
                grid[i][j] = 9; // obstacle
            }
        }
    }

    grid
}

fn bench_dijkstra_uniform_grid(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra_uniform");

    for size in [10, 25, 50, 100].iter() {
        let grid = create_grid(*size);
        let start = (0, 0);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                grid::dijkstra(black_box(&grid), black_box(start), |_, _| Some(1))
            });
        });
    }

    group.finish();
}

fn bench_dijkstra_with_obstacles(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra_obstacles");

    for size in [10, 25, 50, 100].iter() {
        let grid = create_grid_with_obstacles(*size);
        let start = (0, 0);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                grid::dijkstra(black_box(&grid), black_box(start), |_, next| {
                    if *next == 9 {
                        None
                    } else {
                        Some(1)
                    }
                })
            });
        });
    }

    group.finish();
}

fn bench_dijkstra_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra_path");

    for size in [10, 25, 50, 100].iter() {
        let grid = create_grid(*size);
        let start = (0, 0);
        let target = (size - 1, size - 1);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                grid::dijkstra_path(
                    black_box(&grid),
                    black_box(start),
                    black_box(target),
                    |_, _| Some(1),
                )
            });
        });
    }

    group.finish();
}

fn bench_dijkstra_path_with_obstacles(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra_path_obstacles");

    for size in [10, 25, 50, 100].iter() {
        let grid = create_grid_with_obstacles(*size);
        let start = (0, 0);
        let target = (size - 1, size - 1);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                grid::dijkstra_path(
                    black_box(&grid),
                    black_box(start),
                    black_box(target),
                    |_, next| {
                        if *next == 9 {
                            None
                        } else {
                            Some(1)
                        }
                    },
                )
            });
        });
    }

    group.finish();
}

fn bench_dijkstra_variable_costs(c: &mut Criterion) {
    let mut group = c.benchmark_group("dijkstra_variable_costs");

    for size in [10, 25, 50, 100].iter() {
        // Grid with values 1-9 representing different terrain costs
        let grid: Vec<Vec<u32>> = (0..*size)
            .map(|i| (0..*size).map(|j| ((i + j) % 9 + 1) as u32).collect())
            .collect();
        let start = (0, 0);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                grid::dijkstra(black_box(&grid), black_box(start), |_, next| {
                    Some(*next as usize)
                })
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_dijkstra_uniform_grid,
    bench_dijkstra_with_obstacles,
    bench_dijkstra_path,
    bench_dijkstra_path_with_obstacles,
    bench_dijkstra_variable_costs,
);
criterion_main!(benches);
