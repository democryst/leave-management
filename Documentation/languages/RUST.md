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

## 6. Implementation Patterns in LMS

### API Gateway Query Forwarding
When proxying requests with query parameters (e.g., holiday lookups), use `serde_urlencoded` to rebuild the query string from the extracted `serde_json::Value`.

```rust
async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(query): axum::extract::Query<serde_json::Value>,
) -> impl IntoResponse {
    let mut url = format!("{}/api/v1/resource", state.service_url);
    if let Ok(query_str) = serde_urlencoded::to_string(&query) {
        if !query_str.is_empty() {
            url = format!("{}?{}", url, query_str);
        }
    }
    // ... forward request
}
```

### System Actor Pattern
For automated state transitions (like Auto-Approval), use `Uuid::nil()` to signify the "System" as the actor in audit logs. This distinguishes automated actions from manual ones while maintaining type safety.

```rust
if policy.auto_approve {
    request.status = LeaveStatus::Approved;
    request.approver_id = Some(Uuid::nil()); // System User
}
```

---
*Primary backend language for the Leave Management System.*
