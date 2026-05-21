# AI Agent Interaction Rules for Embedded Rust & Automotive Development

## 1. Context & Environment Context
*   **Host Environment:** WSL (Windows Subsystem for Linux), Ubuntu/Debian core.
*   **Target Core:** Embedded Systems / Automotive Architecture (ECU, Microcontrollers).
*   **Standards Domain:** AUTOSAR Classic specification layout (BSW, Layered software architecture, PDU processing, Network Management).
*   **Tooling Preferences:** `cargo`, `syntect` for ANSI 24-bit terminal highlights, clean separation between terminal UI (`main.rs`) and driver logic (`ai_client.rs`).

---

## 2. Code Generation & Constraints

### A. Idiomatic & Resource-Conscious Rust
*   **No Allocation in Drivers:** Unless explicitly requested, assume a resource-constrained environment. Avoid heavy allocations (`String`, `Vec`) inside logic loops; prefer stack-allocated arrays `[u8; N]`, slices `&[u8]`, and static trait objects.
*   **Explicit Error Handling:** Avoid `unwrap()` or `expect()` in core logical snippets. Always leverage idiomatic `Result<T, E>` patterns. Use custom enums or concise `.map_err(|_| "Contextual error")?` closures for upper layer propagation.
*   **Zero-Cost Abstractions:** Prefer generics, static dispatch via traits, and compile-time verification (Template Metaprogramming equivalents in Rust) over dynamic dispatch (`dyn`).

### B. Automotive/AUTOSAR Domain Alignment
*   **Explicit Naming Rules:** Use explicit types matching AUTOSAR naming schemas where possible (e.g., `PduIdType` instead of just `u16`, `NetworkHandleType` instead of `u8`).
*   **Decoupled Architecture:** Emulate the clear separation of layer boundaries (e.g., COM layer doesn't access peripheral registers directly; it coordinates via structural callbacks or intermediate interfaces).

---

## 3. Communication & Explanations Style

*   **No Text Fluff:** Omit introductory filler phrases ("Sure, I can help with that", "Great question!"). Transition straight into the data or concrete code block.
*   **Inline Syntax Explanations:** For advanced or unique Rust components (e.g., lifetimes, combinators like `.map_err(|_| ...)`, raw pointer conversions), append quick, inline parenthetical notes defining *why* it is required in embedded systems.
*   **Keep Markdown Identifiers Clean:** Ensure all generated headers (`##` or `###`) and code blocks are formatted strictly with explicit language tags (` ```rust `, ` 
```toml `) so terminal line parsers like `syntect` can capture syntax segments perfectly without tripping.