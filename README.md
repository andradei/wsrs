# This is a very small project for LEARNING and COMPARISON purposes

## ws - directory alias manager

This is a Rust port of Go project [ws](https://github.com/andradei/ws).

The intention of this project was to:

- Learn more about Rust by writing a tiny project
- Compare the development project with the Go version
- Compare the final code with the Go version (readability, maintainability, patterns, syntax, etc.)

Conclusions:

- Rust can be more readable than Go for small projects
- The compiler has the most useful messages, matching the strictness of ownership and borrowing
- `enum`s are a nice feature to have
- The type systems is very robust, but hard to first understand
- Error handling is still not great (same for Go) even though it is compile-time-checked
- Cargo is a great tool for project and dependency management
- The current module/crate/import system is weird (should be fixed on the next Rust epoch)

This was a great experiment and I learned much more that what is listed above.
