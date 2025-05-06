# Altcha - SHA-256 Challenge Solver

A high-performance Rust implementation of a bidirectional hash challenge solver using parallel processing to find a number that, when combined with a salt and hashed with SHA-256, produces a specific target hash.

## Features

- Bidirectional search (searches from both ends simultaneously)
- Parallel processing with configurable number of workers
- Automatic workload distribution among workers

## How It Works

The solver employs two main strategies:

1. **Bidirectional Search**: Splits the search space into two parts:
    - Forward workers search from 0 → middle
    - Backward workers search from maxNumber → middle

2. **Parallel Processing**:
    - Uses 2 workers (1 forward + 1 backward)
    - Each worker handles a specific range of numbers
    - First found solution terminates all searches