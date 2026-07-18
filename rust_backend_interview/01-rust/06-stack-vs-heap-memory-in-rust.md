# Stack vs Heap Memory in Rust

## Interview Question

Explain Stack vs Heap Memory in Rust.

## Interview Answer

> "Rust uses both stack and heap memory. The stack stores fixed-size data with a known size at compile time, such as integers, booleans, and references. Stack allocation is very fast because memory is managed automatically using a Last-In-First-Out structure.
>
> The heap stores dynamically sized data like `String`, `Vec`, and `HashMap`. Heap allocation is slower because memory must be requested from the operating system, and Rust tracks it through ownership.
>
> When a stack value like `i32` is assigned, it is copied because it has a fixed size. However, heap-allocated types like `String` move ownership instead of copying the heap data to avoid expensive memory copies. This design gives Rust both high performance and memory safety."

---

## Follow-up Questions & Answers

### Q1. What is the difference between Stack and Heap?

**Interview Answer:**

> "The stack stores fixed-size data and provides very fast allocation and deallocation. The heap stores dynamically sized data, requires explicit allocation, and is generally slower but more flexible."

---

### Q2. Why is Stack faster than Heap?

**Interview Answer:**

> "The stack follows a simple Last-In-First-Out structure, so allocation and deallocation only adjust the stack pointer. Heap allocation requires searching for available memory and managing fragmentation."

---

### Q3. Why does `String` use the Heap?

**Interview Answer:**

> "Because its size can change at runtime. Since the compiler doesn't know its final size during compilation, the actual data is stored on the heap."

---

### Q4. Why does `i32` implement `Copy` but `String` does not?

**Interview Answer:**

> "`i32` has a fixed size and can be copied efficiently. `String` owns heap memory, so copying it implicitly would be expensive and could lead to double-free errors. Rust uses ownership transfer instead."

---

### Q5. What is stored on the Stack for a `String`?

**Interview Answer:**

> "The stack stores the pointer, length, and capacity. The actual string data is stored on the heap."

---

### Q6. What happens when a `String` goes out of scope?

**Interview Answer:**

> "Rust automatically calls the `Drop` trait, which releases the heap memory owned by the `String`."

---

### Q7. Why is heap allocation expensive?

**Interview Answer:**

> "Heap allocation involves requesting memory from the allocator, finding a suitable memory block, and maintaining allocation metadata, making it slower than stack allocation."

---

### Q8. How does understanding Stack vs Heap help in backend development?

**Interview Answer:**

> "It helps reduce unnecessary allocations and cloning, resulting in lower memory usage, lower latency, and higher throughput in production services."

---

### Q9. Does every variable live on either the Stack or Heap?

**Interview Answer:**

> "Yes. Variables themselves are typically stored on the stack, but they may own or reference data stored on the heap, depending on the type."

---

### Q10. Why is Stack vs Heap important in Rust interviews?

**Interview Answer:**

> "Because Rust's ownership model, borrowing, moves, cloning, smart pointers, and performance optimizations all depend on understanding where data is stored and how it is managed."
