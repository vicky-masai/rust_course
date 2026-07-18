# Traits

## Interview Question

Explain Traits.

## Interview Answer

"Traits define shared behavior similar to interfaces in other languages."

---

## Follow-up Questions & Answers

### Q1. What is the difference between trait objects and generics?

**Interview Answer**

Generics use monomorphization—the compiler generates a specialized function for each concrete type, giving zero-cost abstraction but larger binaries. Trait objects use dynamic dispatch via a vtable pointer, adding runtime overhead but allowing heterogeneous collections like `Vec<Box<dyn Draw>>`. Choose generics for performance, trait objects for flexibility.

---

### Q2. What is trait inheritance and how does it work?

**Interview Answer**

A trait can require implementors to also implement another trait, like `trait Serialize: Display { fn serialize(&self) -> String; }`. This means any type implementing `Serialize` must also implement `Display`. It's similar to interface extension in Java, and Rust uses `where` bounds to express complex trait hierarchies in generic constraints.

---

### Q3. Can you implement a foreign trait for a foreign type?

**Interview Answer**

No, Rust's orphan rule prevents implementing a trait you didn't define for a type you didn't define. This avoids conflicting implementations across crates. The workaround is the newtype pattern—wrap the foreign type in a local struct and implement the trait for that. The `derive_more` and `newtype_derive` crates help with this.

---

### Q4. What is the `Display` trait and why is it important?

**Interview Answer**

`Display` provides a human-readable string representation via the `{}` formatter. It's used for user-facing output like error messages and logging. Implementing `Display` for your error types makes them work with `?` in functions returning `anyhow::Error` or `Box<dyn Error>`, since the error chain can be displayed to users.

---

### Q5. What is the difference between `impl Trait` and `dyn Trait`?

**Interview Answer**

`impl Trait` in argument position is syntactic sugar for a generic parameter—the type is resolved at compile time. `dyn Trait` creates a trait object with dynamic dispatch. In return position, `impl Trait` hides the concrete type (useful for iterators), while `dyn Trait` explicitly boxes the value with a vtable.

---

### Q6. How do default method implementations work?

**Interview Answer**

Traits can provide method bodies that implementors can optionally override. For example, `Iterator::size_hint` defaults to `(0, None)`. This lets you add functionality without breaking existing implementations. However, default methods can't access `self` by reference in some cases and can't add new associated types.

---

### Q7. What are associated types vs generic parameters on traits?

**Interview Answer**

Associated types fix one type per trait implementation, like `type Item` in `Iterator`. Generic parameters allow multiple implementations per type, like `impl From<String> for MyType` and `impl From<i32> for MyType`. Use associated types when there's a natural one-to-one mapping, generics when you need multiple implementations.

---

### Q8. How do negative trait bounds work (e.g., `!Send`)?

**Interview Answer**

Rust doesn't support negative bounds in `where` clauses yet, but you can use `PhantomData<*const ()>` to make a type `!Send` and `!Sync`. This is useful for creating types that are intentionally non-thread-safe, like `Rc<T>`. The `auto_traits` feature in nightly allows opting out of auto traits more explicitly.

---

### Q9. What is the `Sized` trait and why does it matter?

**Interview Answer**

`Sized` is an auto-trait that means the type's size is known at compile time. All generic type parameters are `Sized` by default. DSTs (dynamically sized types) like `str` or `[T]` are `!Sized` and must be behind a pointer like `&str` or `Box<[T]>`. This affects how you write generic functions and struct definitions.

---

### Q10. How do traits enable the builder pattern in Rust?

**Interview Answer**

Traits allow flexible builder APIs through method chaining and type-state patterns. You can use a trait to define builder methods that return `Self`, and use generic bounds to enforce that required fields are set before building. The type-state pattern uses phantom types to enforce valid state transitions at compile time.

---
