# Altcha - SHA-256 Challenge Solver

A high-performance Go implementation of a bidirectional hash challenge solver using parallel processing to find a number that, when combined with a salt and hashed with SHA-256, produces a specific target hash.

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
    - Uses 20 workers (10 forward + 10 backward)
    - Each worker handles a specific range of numbers
    - First found solution terminates all searches

## Usage

```go
challenge := Challenge{
    Algorithm:  "SHA-256",
    Challenge:  "<target-hash>",
    MaxNumber: 150000,
    Salt:      "<salt-string>",
}

result := solveChallenge(challenge)
```

## Perfomance 

The official documentation states the following benchmarks for complexity of 100,000:

| Device | Performance | Time to Solve |
|--------|------------|---------------|
| MacBook Pro M3-Pro (2023) | 3 ops/s | 0.33 sec |
| iPhone 12 mini (2020) | 1.2 ops/s | 0.83 sec |
| AWS EC2 (c6a.xlarge) | 1 ops/s | 1 sec |
| Samsung Galaxy A14 (2023) | 0.4 ops/s | 2.5 sec |
| AWS Lambda (1GB) | 0.12 ops/s | 8 sec |


My implementation needs only ~8ms