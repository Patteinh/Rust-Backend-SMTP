# Rust-Backend <img src="https://skillicons.dev/icons?i=rust" />

## Table of Contents 📑

-   [Overview of Rust-Backend-smtp Backend](#overview-of-rust-backend-smtp-backend-)
-   [Technology Stack](#technology-stack-)
-   [Project Structure](#project-structure-)
-   [Dependencies](#dependencies-)
-   [Getting Started with Rust-Backend-smtp Backend](#getting-started-with-rust-backend-smtp-backend-)
-   [License](#license-)

<br>

## Overview of Rust-Backend-smtp Backend 🔎

The backend of Rust-Backend-smtp is responsible for handling server-side logic, managing APIs, and integrating with external services. It provides functionalities such as email handling, CORS setup, and route configuration to ensure a secure and reliable experience for users interacting with the backend.

### Main Objectives:

-   Implement server-side logic to handle incoming requests and serve appropriate responses.
-   Ensure security measures are in place to protect against common web vulnerabilities.
-   Integrate with external services for email communication and data handling.

### Key Features:

-   Actix Web Framework: Utilizes Actix Web, a powerful, pragmatic, and extremely fast web framework for Rust, for building robust APIs and handling HTTP requests.
-   Middleware: Employs various middleware functions for handling CORS, CSRF protection, error handling, and rate limiting.
-   Lettre: Integrates Lettre for sending emails, providing a reliable solution for email communication.
-   Configuration: Modularizes project configuration to keep settings organized and manageable.

### General Architecture:

-   Backend Server: The backend server built with Actix Web, responsible for handling API requests and serving responses.
-   Middleware Layer: Middleware functions used for processing incoming requests, applying security measures, and handling errors.
-   Controllers: Modules containing logic for handling specific routes and business logic, such as sending emails.

<br>

## Technology Stack 💻

<p align="center">
    <img src="https://skillicons.dev/icons?i=rust" />
</p>

This section describes the key technologies used for building Rust-Backend-smtp Backend, covering server-side development and API management.

<br>

### Backend:

-   Rust: Systems programming language known for its performance, reliability, and memory safety.
-   Actix Web: Powerful, pragmatic, and extremely fast web framework for Rust, chosen for its performance and simplicity.
-   Tokio: Asynchronous runtime for Rust, providing a fast, reliable, and scalable foundation for building asynchronous applications.

<br>

## Project Structure 📂

```
Rust-Backend-smtp
├── config
│   ├── mod.rs
│   └── smtp.rs
├── controllers
│   ├── greet.rs
│   ├── mod.rs
│   └── send_email.rs
├── data
│   ├── app_state.rs
│   ├── email_data.rs
│   └── mod.rs
├── main.rs
├── middleware
│   ├── cors.rs
│   ├── logger.rs
│   └── mod.rs
├── routes
│   ├── greet.rs
│   ├── mod.rs
│   └── send_email.rs
└── templates
    ├── email.rs
    ├── email_template.txt
    └── mod.rs
```

This structure represents the organization of files and directories in the Rust-Backend-smtp project.

<br>

## Dependencies 📚

### Actix Web Framework:

-   **actix-cors** (version 0.7.0): Middleware for enabling CORS (Cross-Origin Resource Sharing) in Actix Web applications.
-   **actix-service** (version 2.0.2): Provides traits and utilities for working with services in Actix applications.
-   **actix-web** (version 4.5.1): The main Actix Web framework for building web applications in Rust.

### Email Handling:

-   **dotenv** (version 0.15.0): Loads environment variables from a .env file into the environment.
-   **lettre** (version 0.11.7): A mailer library for Rust, used for sending emails.

### Asynchronous Runtime:

-   **native-tls** (version 0.2.11): Native TLS bindings for Rust based on the OpenSSL library.
-   **tokio** (version 1.37.0): An asynchronous runtime for Rust, providing a fast, reliable, and scalable foundation for building asynchronous applications.

### Serialization:

-   **serde** (version 1.0.200): A serialization/deserialization library for Rust, used for data interchange format.

<br>
## Getting Started with Rust-Backend-smtp Backend 🚀

To get started with Rust-Backend-smtp Backend, follow these steps:

1. Clone the repository: `git clone <repository-url>`
2. Navigate to the project directory: `cd Rust-Backend-smtp`
3. **Set up environment variables:**
    - Create a `.env` file in the project root.
    - Define the following environment variables in the `.env` file:
        - `FRONTEND_URL`: The URL of the frontend application that will interact with the backend.
        - `EMAIL_USER`: The email address used for sending emails.
        - `EMAIL_PASSWORD`: The password associated with the email address for authentication.
        - `BACKEND_ADDRESS`: The address at which the backend server will be accessible. If not specified, defaults to `127.0.0.1:8080`.
        - `BACKEND_ADDRESS_LOCAL`: The local address at which the backend server will be accessible. If not specified, defaults to `localhost:8080`.
4. **Install dependencies:**
    ```shell
    cargo build
    ```
5. **Run the server:**
    ```shell
    cargo run
    ```
6. **Access the backend APIs** at the specified server address.

### Running with Optimization:

For production deployment or optimized performance, you can build and run the server with optimized settings:

1. **Build the project with optimization:**

    ```shell
    cargo build --release
    ```

    This command compiles the project with optimizations enabled.

2. **Run the server with optimized settings:**

    ```shell
    cargo run --release
    ```

    This command executes the server with optimizations enabled for improved performance.

3. **Access the backend APIs** at the specified server address.

## License

This project is licensed under the [MIT License](LICENSE).
