# MIR

## Interview Question

Explain MIR.

## Interview Answer

"MIR is Rust's intermediate representation. It simplifies borrow checking, optimization, and code generation before converting to LLVM IR."

---

## Follow-up Questions & Answers

### Q1. Why was MIR introduced in Rust?

**Interview Answer**

MIR was introduced to provide a simpler representation for the borrow checker and to enable Rust-specific optimizations. The AST is too complex for efficient analysis, and jumping straight to LLVM IR loses Rust-specific information. MIR sits in the middle, being simple enough for borrow checking while retaining enough structure for optimization.

---

### Q2. What does MIR look like compared to AST?

**Interview Answer**

MIR is a control-flow graph of basic blocks with explicit temporaries, lifetimes, and borrow points. It desugars `if let`, `match`, and `for` loops into explicit branches. Each statement is simplified—no complex expressions, just assignments, projections, and calls. This makes it much easier to analyze than the rich AST.

---

### Q3. How does MIR enable better borrow checking?

**Interview Answer**

MIR represents borrows as explicit `Ref` and `Deinit` statements at specific points in the CFG. The borrow checker operates on this simplified form, tracking where each loan is created, used, and dropped. This point-based analysis is more precise than lexical analysis and enables NLL and two-phase borrowing.

---

### Q4. What MIR optimizations does rustc perform?

**Interview Answer**

MIR optimizations include constant propagation, dead code elimination, inlining, and copy propagation. These are cheaper than LLVM optimizations and can expose more opportunities for LLVM. For example, MIR inlining can eliminate function call overhead for small functions, allowing LLVM to optimize the resulting code more effectively.

---

### Q5. How can you view the MIR for your code?

**Interview Answer**

Use `cargo rustc -- --emit=mir` to output MIR to `mir_dump/`. The `-Zdump-mir=all` flag (nightly only) dumps MIR at various compilation stages. MIR is also printed in error messages when `#![feature(rustc_attrs)]` is enabled. The MIR is written in a textual format showing basic blocks, locals, and statements.

---

### Q6. What is the difference between MIR and THIR?

**Interview Answer**

THIR (Typed High-Level IR) is an intermediate form between AST/HIR and MIR. It resolves types and performs type checking before lowering to MIR. THIR is more structured than HIR but less simplified than MIR. Type checking happens on THIR, while borrow checking happens on MIR, creating a clean separation of concerns.

---

### Q7. How does MIR handle pattern matching?

**Interview Answer**

MIR desugars complex pattern matches into sequences of `SwitchInt`, `Assert`, and `FakeBorrow` statements. Each match arm becomes a basic block with appropriate guards. The MIR builder generates decision trees for match expressions, and the optimizer can simplify these into more efficient jump sequences.

---

### Q8. What is MIR-based const evaluation?

**Interview Answer**

Rust can evaluate `const fn` and static initializers using the MIR interpreter at compile time. This runs MIR statements directly, evaluating expressions and function calls in a compile-time context. It supports arithmetic, array operations, and basic control flow, enabling `const` arrays, `const` generics, and const trait methods.

---

### Q9. How does MIR interact with codegen?

**Interview Answer**

After MIR optimizations, the codegen backend converts MIR to LLVM IR. Each MIR basic block maps to an LLVM basic block. MIR locals become LLVM registers or stack allocations. MIR's explicit borrow points become lifetime annotations in LLVM IR, allowing LLVM to optimize memory operations more effectively.

---

### Q10. Can MIR be used for static analysis tools?

**Interview Answer**

Yes, tools like `cargo-miri` (for detecting UB in unsafe code) and `rust-analyzer` use MIR for analysis. MIR's simplified form makes it ideal for detecting undefined behavior, data flow analysis, and dead code detection. The `mir_dump` output is also used by the Rust compiler team for debugging compilation issues.

---
