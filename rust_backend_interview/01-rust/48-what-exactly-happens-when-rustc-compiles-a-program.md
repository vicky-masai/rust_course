# What exactly happens when `rustc` compiles a program?

## Interview Question

What exactly happens when `rustc` compiles a program?

## Interview Answer

1. Parsing
2. Macro expansion
3. Name resolution
4. Type checking
5. Borrow checking
6. MIR (Mid-level Intermediate Representation)
7. Optimizations
8. LLVM IR generation
9. LLVM optimization
10. Machine code generation
11. Linking

---

## Follow-up Questions & Answers

### Q1. What is the role of the parser in rustc?

**Interview Answer**

The parser converts source code text into an Abstract Syntax Tree (AST). It handles Rust's syntax including macros, generics, lifetimes, and attributes. The parser produces error messages for syntax violations early, before any semantic analysis. It uses a recursive descent approach with Pratt parsing for expressions.

---

### Q2. How does macro expansion work in the compilation pipeline?

**Interview Answer**

Macro expansion happens after parsing, converting `macro_rules!` and procedural macros into regular Rust code. Declarative macros pattern-match on token trees and produce new tokens. Procedural macros receive a `TokenStream` and return transformed code. Expansion is iterative—macros can produce new macros that are expanded in subsequent passes.

---

### Q3. What is name resolution and when does it happen?

**Interview Answer**

Name resolution maps identifiers to their definitions after macro expansion. It resolves `use` statements, method calls, trait implementations, and associated types. It runs before type checking and handles Rust's complex name resolution rules including glob imports, shadowing, and trait path disambiguation.

---

### Q4. How does type checking work in rustc?

**Interview Answer**

Type checking verifies that expressions have consistent types and that trait bounds are satisfied. It unifies types using a constraint-based algorithm, solving type variables through unification. Generic functions are checked against all concrete usages. Type inference allows omitting type annotations in most cases, with the compiler deducing types from context.

---

### Q5. What optimizations does rustc perform before LLVM?

**Interview Answer**

Rust's own optimizations include constant folding, dead code elimination, and MIR-level optimizations like inlining and loop unrolling. These happen on MIR before LLVM IR generation. MIR optimizations are cheaper than LLVM's and can enable further LLVM optimizations. The `-O` flag enables both Rust and LLVM optimization passes.

---

### Q6. What is the difference between `cargo build` and `rustc` directly?

**Interview Answer**

`cargo build` manages dependencies, invokes `rustc` with correct flags, handles incremental compilation, and caches build artifacts. Running `rustc` directly requires specifying all dependencies and flags manually. Cargo also runs `rustfmt` and `clippy` if configured, and handles workspace-level parallel builds automatically.

---

### Q7. How does incremental compilation work?

**Interview Answer**

Incremental compilation caches intermediate results (like MIR and LLVM IR) keyed by the query that produced them. When you rebuild, only queries whose inputs changed are re-executed. This is managed by the query system, which tracks dependencies between compilation steps. It significantly speeds up edit-compile cycles during development.

---

### Q8. What are procedural macros and when are they expanded?

**Interview Answer**

Procedural macros are Rust functions that run at compile time, transforming `TokenStream` into `TokenStream`. They include derive macros (`#[derive(Serialize)]`), attribute macros (`#[route(GET, "/")]`), and function-like macros. They're expanded during the macro expansion phase, before type checking, and can generate arbitrary Rust code.

---

### Q9. How does the linker step work in rustc?

**Interview Answer**

After generating machine code for each crate, the linker combines object files and resolves external symbols. Rust uses the system linker (ld, lld, or link.exe). Static linking embeds all dependencies, while dynamic linking references shared libraries. The `-C prefer-dynamic` flag influences this choice. Linking is often the slowest step in large projects.

---

### Q10. What is the query system in rustc?

**Interview Answer**

The query system is rustc's internal dependency tracking mechanism. Each compilation step is a query (e.g., `type_of(def_id)`) that caches its result and tracks which other queries depend on it. When source code changes, only affected queries are re-executed. This enables both incremental compilation and parallel compilation within a single crate.

---
