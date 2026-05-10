# Language Reference: Rust

Rust is the language of reliability and performance. It provides memory safety comparable to Java or Python but with the "bare metal" performance of C or C++. It is the primary language used for the backend services in the Leave Management System.

---

## 1. Core Philosophy: "Safety Without Sacrifice"
Rust eliminates manual memory management pitfalls (leaks, double-frees) without relying on a Garbage Collector.
- **The Borrow Checker:** The heart of Rust's memory safety.
- **Ownership:** Each value has a single owner. Memory is cleaned up automatically when the owner goes out of scope.
- **Borrowing (&):** Lending values. Rules: Either one mutable reference OR multiple immutable references—never both simultaneously.

## 2. Key Technical Features
- **Zero-Cost Abstractions:** No Garbage Collector (GC). Cleanup code is inserted at compile-time.
- **Fearless Concurrency:** The borrow checker makes data races physically impossible to compile.
- **Algebraic Data Types:** Powerful Enums that can hold data.
- **No Nulls:** Uses `Option<T>` and `Result<T, E>` to force explicit handling of "None" and "Error" cases, preventing Null Pointer Exceptions.

## 3. The Ecosystem (Cargo)
- **Package Management:** Handles "crates" (libraries).
- **Build System:** Highly optimized compilation.
- **Testing:** Native support for unit and integration tests.
- **Documentation:** Auto-generated HTML from code comments.

## 4. Comparison: Rust vs. Go
| Feature | Go | Rust |
|---------|----|------|
| **Learning Curve** | Low | High (Strict Borrow Checker) |
| **Performance** | Excellent | Peak (Bare Metal) |
| **Memory** | Garbage Collector | Ownership (No GC) |
| **Concurrency** | Goroutines/Channels | Threads/Async (Race-free) |

## 5. Why We Use Rust for LMS
- **Reliability:** The compiler prevents the most expensive bugs before deployment.
- **Performance:** Critical for our HR logic and precision math (`BigDecimal`).
- **Safety:** Ensuring PII (Staff data) is handled without risk of memory exploits.

---
*Primary backend language for the Leave Management System.*
