# stellar-prime

Smart contract for prime numbers in Rust using the Soroban SDK.

## Overview

This repository contains a smart contract that determines if a number is prime.
- If the number is negative, it returns `"0"`.
- If the number is prime, it returns:
  
  ```
  "El número X es primo."
  ```
  
- If the number is not prime, it returns:
  
  ```
  "El número X no es primo. Factores: F1 F2 F3 ..."
  ```
  
  where the factors (divisors) of the number are listed.

## Tests

The repository includes unit tests that check the contract's functionality. To run the tests, simply execute:

```bash
cargo test
```

### The tests cover several scenarios including:
- Negative numbers
- Special cases (0 and 1)
- Prime numbers (e.g., 2, 7)
- Composite numbers (e.g., 4, 9) with factor listing

## Getting Started

### Clone the repository:

```bash
git clone https://github.com/ckshei/stellar-prime.git
```

### Navigate into the project directory:

```bash
cd stellar-prime
```

### Run the tests:

```bash
cargo test
```

## Requirements

- **Rust** (latest stable version)
- **Soroban SDK** (check the `Cargo.toml` for version details)

## License feel free to copy my homework