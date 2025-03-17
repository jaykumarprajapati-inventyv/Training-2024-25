# Axum + TiDB + Couchbase Authentication & Product Management

## Overview
This project implements authentication using Axum, TiDB, and Couchbase with password hashing (bcrypt) and JWT-based authentication. After successful login, users can manage their own products (CRUD operations).

## Features
- **User Registration**: Stores username, email, and hashed password in TiDB & Couchbase.
- **User Login**: Validates credentials and returns a JWT token.
- **JWT Authentication**: Protects routes using middleware.
- **Product Management**: Users can add, update, delete, and list their own products.

## Technologies Used

- Rust (Programming Language)

- Axum (Web Framework)

- SQLx (Database Querying)

- Couchbase (NoSQL Database)

- TiDB (SQL Database)

- JWT (jsonwebtoken) (Token-Based Authentication)

- bcrypt (Password Hashing)

- tokio (Async Runtime)


## Setup Instructions
### (1️) Prerequisites
- Install **Rust & Cargo**
- Install **Couchbase** & **TiDB**
- Setup **.env** file with required database credentials

### (2️) Clone the Repository
```sh
git clone link
cd repo_name
```

### (3️) Install Dependencies
```sh
cargo build
```

### (4️) Run the Project
```sh
cargo run
```

## API Endpoints

### **User Registration**
**POST** `/register`

### **User Login**
**POST** `/login`

### **Protected Routes (Require JWT in `Authorization` Header)**

### **User Dashboard**
**GET** `/dashboard`

#### **Add Product**
**POST** `/products`

#### **Get All Products of Logged-in User**
**GET** `/products`

#### **Update Product**
**PUT** `/products/{id}`

#### **Delete Product**
**DELETE** `/products/{id}`

#### **Delete All Products of Logged-in User**
**DELETE** `/products`

## Middleware (JWT Authentication)
- **Every request (except login & register) is checked for a valid JWT token.**
- Token is validated, and user ID is extracted to ensure they only manage their own products.

## Run the Project
```sh
cargo run
```

Now, users can register, log in, and manage their own products securely! 

## Folder Structure

```
own_product/                    # Root project directory
│
├── src/                         # Source code directory
│   │
│   ├── database/                # Database-related logic
│   │
│   ├── handlers/                # API route handlers
│   │   ├── registration.rs       # Handles user registration
│   │   ├── user_dashboard.rs     # Handles user dashboard route
│   │   ├── user_login.rs         # Handles user login
│   │   └── mod.rs                # Re-exports handler modules
│   │
│   ├── middlewares/             # Middleware for authentication and validation
│   │   ├── auth_middleware.rs    # JWT authentication middleware
│   │   └── mod.rs                # Re-exports middleware modules
│   │
│   ├── models/                  # Defines database models
│   │   ├── model.rs              # User and product models
│   │   └── mod.rs                # Re-exports model modules
│   │
│   ├── router/                  # Route configurations
│   │   ├── jwt_utils.rs          # JWT creation and validation utilities
│   │   ├── mod.rs                # Re-exports router modules
│   │   └── main.rs               # Application entry point
│
├── target/                      # Compiled binaries and build cache
│
├── .env                         # Environment variables (DB URL, secrets, etc.)
├── .gitignore                   # Git ignore file
├── Cargo.lock                   # Cargo dependency lock file
├── Cargo.toml                   # Rust project dependencies
└── README.md                    # Documentation for the project


```