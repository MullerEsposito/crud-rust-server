
# CRUD Rust Server

## 📋 Project Overview

This project is a basic CRUD (Create, Read, Update, Delete) server built with Rust. It provides RESTful API endpoints for managing a collection of people. The application structure follows a modular architecture, making it scalable and maintainable.

## 🚀 Features

- **Create** new person entries
- **Read** single or multiple person entries
- **Update** existing person data
- **Delete** person entries

## ⚙️ Technologies Used

- **Rust**: Programming language for system-level development
- **Actix-web**: Powerful, pragmatic, and extremely fast web framework for Rust
- **SQLx**: Async SQL crate for Rust for database interactions
- **PostgreSQL**: Relational database management system

## 📂 Project Structure

```
├── src
│   ├── db.rs               # Database connection and setup
│   ├── main.rs             # Entry point of the application
│   ├── types.rs            # Custom data types
│   ├── models              # Data models
│   │   ├── mod.rs
│   │   └── person.rs
│   ├── people              # CRUD functionalities for person entity
│   │   ├── controllers     # Business logic for endpoints
│   │   └── services        # Interaction with the database
│   ├── routes              # Route definitions
│   │   ├── mod.rs
│   │   ├── people_routes.rs
│   │   └── routes.rs
│
├── Cargo.toml              # Project configuration
├── LICENSE                 # License information
├── .gitignore              # Ignored files for git
└── README.md               # Project documentation
```

## 🛠️ Installation and Usage

### Prerequisites

- Rust (Install from [rustup.rs](https://rustup.rs/))

### Setup Steps

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd crud-rust-server
   ```
2. **Run the server**
   ```bash
   cargo run
   ```

### API Endpoints

- `POST /people` - Create a new person
- `GET /people` - Retrieve all persons
- `GET /people/{id}` - Retrieve a specific person
- `PUT /people/{id}` - Update a specific person
- `DELETE /people/{id}` - Delete a specific person

## 📝 License

This project is licensed under the MIT License.

## 🙌 Contributing

Contributions are welcome! Please open an issue or submit a pull request for any changes.

## 📧 Contact

For questions or suggestions:

- **Name:** Müller Esposito Nunes
- **Email:** mulleresposito@hotmail.com
- **LinkedIn:** [linkedin.com/in/mulleresposito](https://linkedin.com/in/mulleresposito)
