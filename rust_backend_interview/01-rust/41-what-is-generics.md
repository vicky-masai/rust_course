# What is Generics?

## Interview Question

What is Generics?

## Interview Answer

"Generics allow writing reusable code that works with multiple data types."

---

## Follow-up Questions & Answers

### Q1. How does monomorphization work in Rust?

**Interview Answer**

The compiler generates a specialized copy of each generic function for every concrete type used. For example, `fn max<T: Ord>(a: T, b: T) -> T` becomes `max_i32`, `max_string`, etc. This gives zero-cost abstraction—no runtime dispatch—but increases binary size. The compiler can often inline and optimize away unused specializations.

---

### Q2. What are const generics?

**Interview Answer**

Const generics let you parameterize types by constant values, not just types. `struct Array<T, const N: usize>` creates fixed-size arrays as generic types. This enables `Array<i32, 5>` and `Array<String, 10>` with compile-time size checking, useful for fixed-capacity buffers and SIMD operations where the size must be known at compile time.

---

### Q3. Can you have generic methods on a non-generic struct?

**Interview Answer**

Yes, methods can be generic even if the struct isn't. For example, `impl Vec<u8> { fn push_generic<T: Into<u8>>(&mut self, val: T) { ... } }` adds a generic method to `Vec<u8>`. This is common in serde where `deserialize` is generic over the `Deserializer` type, allowing the same struct to be deserialized from JSON, TOML, etc.

---

### Q4. What is the difference between `T: Trait` and `impl Trait`?

**Interview Answer**

`T: Trait` in a function signature declares a generic parameter bounded by the trait. `impl Trait` is syntactic sugar that creates an anonymous generic parameter. They're equivalent in argument position: `fn foo<T: Display>(x: T)` and `fn foo(x: impl Display)` compile to the same monomorphized code.

---

### Q5. How do you handle multiple trait bounds on generics?

**Interview Answer**

Use `where` clauses for readability: `fn process<T>(item: T) where T: Display + Clone + Send { ... }`. The `+` syntax works inline but becomes unreadable with many bounds. `where` clauses also enable more complex bounds like `where T::Item: Debug` for associated types, which inline bounds can't express.

---

### Q6. What are phantom types and how do they use generics?

**Interview Answer**

Phantom types are generic parameters not used in the struct's fields but tracked at compile time. `struct Request<Stage> { url: String, _stage: PhantomData<Stage> }` lets you create `Request<Unsent>` and `Request<Sent>` as different types, preventing use-after-send at compile time without runtime cost.

---

### Q7. Can generics be used for operator overloading?

**Interview Answer**

Yes, through traits like `std::ops::Add`, `Mul`, etc. Implementing `Add for MyType` lets you use `+` syntax. Generics allow writing functions like `fn dot_product<T: Num>(a: &[T], b: &[T]) -> T` that work with any numeric type. This is how ndarray and nalgebra support multiple numeric precisions.

---

### Q8. What is turbofish syntax and when is it needed?

**Interview Answer**

Turbofish `::<>` specifies generic parameters explicitly: `"42".parse::<i32>()`. It's needed when the compiler can't infer the type, like in `collect::<Vec<_>>()` or `std::iter::empty::<i32>()`. It's also useful for disambiguating method calls when multiple trait implementations match.

---

### Q9. How do generics affect compilation time?

**Interview Answer**

Monomorphization increases compilation time because each specialization must be type-checked and optimized independently. A function used with 10 types generates 10 copies of LLVM IR. Strategies to mitigate this include using trait objects for dynamic dispatch, minimizing generic depth, and using `cargo-clippy` to identify unnecessary generics.

---

### Q10. How do generics interact with trait objects for dynamic dispatch?

**Interview Answer**

You can't directly create a `Vec<dyn Trait>` without boxing, because trait objects are unsized. The pattern `Vec<Box<dyn Trait>>` stores fat pointers (data + vtable) on the heap. Generics over trait objects enable heterogeneous collections but lose the ability to call methods that require `Self: Sized`, limiting the API surface.

---
