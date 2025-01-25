# Owner Fee Management System

This project is a Rust-based application for managing owner fees, including creating, updating, and calculating various types of fees.

## Features

- Create new fee records
- Update existing fee records
- Calculate total fees
- Perform self-joins using Diesel
- Group streams of items using `itertools`
- Handle transactions with custom traits

## Prerequisites

- Rust
- Cargo
- PostgreSQL (or your preferred database)
- Diesel CLI

## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/AndrewJiao/owner-fee-management.git
    cd owner-fee-management
    ```

2. Install dependencies:

    ```sh
    cargo build
    ```

3. Set up the database:

    ```sh
    diesel setup
    ```

## Usage

### Running the Application

To run the application, use:

```sh
cargo run
