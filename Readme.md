# Competency Test: P2POOL

This project demonstrates how to generate and submit Bitcoin transactions using Rust and JSON-RPC. The workflow includes:

1. **Generating a Coinbase Transaction:**  
   Mining a block that produces a coinbase transaction (the block reward).

2. **Spending the Coinbase Output:**  
   Creating a transaction (txn1) that spends the matured coinbase output.

3. **Chaining Transactions:**  
   Building a second transaction (txn2) that spends the output from the first spending transaction.

All operations are performed via the Bitcoin Core JSON-RPC interface, and the blockchain environment is managed using Docker Compose.

## Prerequisites

- **Linux Environment / WSL:**  
  A Linux system or WSL (Windows Subsystem for Linux) with a proper Rust toolchain installed in the Linux environment.

- **Docker & Docker Compose:**  
  Install [Docker](https://www.docker.com/) and [Docker Compose](https://docs.docker.com/compose/).

- **Rust Toolchain:**  
  Install Rust using [rustup](https://rustup.rs/).  
  *For WSL users:*
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env

## How to Run
- **Clone the repo**
  ```bash
  git clone <repository-url>
  cd <repository-directory>

- **Make the Script Executable:**
   ```bash
   chmod +x run.sh
- **Run the Script:**
  ```bash
  ./run.sh

## Troubleshooting

- **Cargo Command Not Found in WSL:**
  Ensure you have installed Rust inside your WSL environment as described above.

- **Docker Connection Errors:**
  Verify Docker and Docker Compose are installed and running by using:
