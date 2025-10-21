# WASMBD: WebAssembly-based Remote Management Framework

## Project Overview

WASMBD (WebAssembly-based Remote Management Framework) is a professional, secure, and cross-platform solution designed for remote execution, automation, monitoring, and distributed task orchestration. Leveraging the power of WebAssembly (WASM) and WASI (WebAssembly System Interface), it provides a sandboxed, portable, and high-performance environment for executing workloads and modules across diverse operating systems and architectures.

## Key Features

*   **Cross-Platform Compatibility:** Seamless operation across Windows, macOS, Linux, Android, iOS, and major architectures (x86, x86_64, ARM, ARM64).
*   **WASM-Native Backend Agents:** Agents designed for authorized remote execution within secure, isolated WebAssembly runtimes.
*   **Secure by Design:** Encrypted communication (TLS/HTTPS), authentication tokens, access control policies, and comprehensive audit logging.
*   **Modular Plugin Interface:** Dynamically load and run WASM modules for tasks like system monitoring, file operations, diagnostics, and performance benchmarking.
*   **Centralized Orchestration:** A controller/server for managing connected agents, handling registration, updates, and job distribution.
*   **Dynamic Policy Enforcement:** Real-time telemetry and fine-grained permission management for authorized and traceable operations.
*   **Scalable Architecture:** Built with Rust for performance and safety, with containerized deployment (Docker/Kubernetes) for scalability.
*   **Web-based Dashboard & APIs:** Includes a web-based dashboard for management and a REST/gRPC API for programmatic integration.

## Technology Stack

*   **Core Language:** Rust (for performance, safety, and WebAssembly ecosystem).
*   **WebAssembly Runtime:** Wasmtime (for sandboxed execution of WASM modules).
*   **WASI:** WebAssembly System Interface for system-level capabilities.
*   **Controller API:** Axum (Rust web framework) for REST API.
*   **Asynchronous Runtime:** Tokio (Rust asynchronous runtime).
*   **Data Serialization:** Serde (for efficient data serialization/deserialization).
*   **Unique IDs:** UUID (for generating universally unique identifiers).

## Project Structure

The project is organized into a Rust workspace with the following main components:

*   **`agent/`**: The core agent application that runs on target machines, responsible for executing WASM modules.
*   **`controller/`**: The central server that orchestrates agents, manages tasks, and exposes the management API.
*   **`plugins/`**: Contains example and core WebAssembly modules (plugins) that can be dynamically loaded and executed by agents.

## Getting Started (Phase 1: Core Agent and WASM Module Execution)

This section outlines how to build and run the core agent and a sample WASM module.

### Prerequisites

*   Rust and Cargo (ensure `cargo` is in your PATH).
*   `wasm32-wasip1` target installed: `rustup target add wasm32-wasip1`

### Build and Run

1.  **Build the WASM Plugin:**
    ```bash
    cargo build --target wasm32-wasip1 -p hello-world
    ```

2.  **Run the Agent:**
    ```bash
    cargo run -p agent
    ```
    You should see output similar to:
    ```
    Agent: Starting up...
    Agent: Calling WASM module...
    Hello from WASM!
    Agent: WASM module call finished.
    ```

## Controller API (Phase 2: Web Dashboard and API - In Progress)

The controller provides a REST API for managing agents and tasks. To run the controller:

```bash
cargo run -p controller
```

The controller will listen on `0.0.0.0:3000`. You can then interact with the following endpoints:

*   `GET /`: Returns "Hello, Controller!"
*   `GET /agent/register`: Registers a new agent and returns its ID.
*   `GET /agent/status`: Returns the status of all registered agents.
*   `GET /task/submit`: Submits a new task and returns its ID.

## Future Plans

Refer to `PLAN.md` for the detailed development roadmap, including security enhancements, dynamic plugin loading, web dashboard development, and containerized deployment.