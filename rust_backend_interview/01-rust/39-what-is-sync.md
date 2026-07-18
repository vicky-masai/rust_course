# What is Sync?

## Interview Question

What is Sync?

## Interview Answer

"Sync means multiple threads can safely access a reference to the same value."

---

## Follow-up Questions & Answers

### Q1. What is the relationship between `Send` and `Sync`?

**Interview Answer**

If `T: Sync`, then `&T: Send`—meaning shared references can be sent to other threads. Conversely, `T: Send` doesn't imply `T: Sync`. For example, `Cell<T>` is `Send` but not `Sync` because interior mutability without synchronization is unsafe across threads. `Sync` is essentially about the safety of shared references.

---

### Q2. Which types are automatically `Sync`?

**Interview Answer**

Types with only `Sync` fields are auto-`Sync`, including primitives, `String`, `Vec<T>`, `Arc<T>`, and `Mutex<T>`. Types with interior mutability like `Cell<T>` or `Rc<T>` are not `Sync`. You can verify with `fn assert_sync<T: Sync>() {}` and the compiler will error if the type isn't thread-safe for shared access.

---

### Q3. Why is `Cell<T>` not `Sync`?

**Interview Answer**

`Cell<T>` provides interior mutability via `get` and `set` without synchronization primitives. If two threads read and write simultaneously, you get a data race. `RefCell<T>` has the same problem—it uses runtime borrow checking that isn't thread-safe. For thread-safe interior mutability, use `Mutex<T>` or `AtomicXxx` types.

---

### Q4. How does `Sync` affect `tokio::spawn`?

**Interview Answer**

When you send a future to `tokio::spawn`, the future and everything it captures must be `Send`. If the future holds `&T`, that `T` must be `Sync` so the reference can be sent across threads. This is why `Arc<T>` (which is `Sync` when `T: Send + Sync`) works but `Rc<T>` doesn't—it's not `Send` or `Sync`.

---

### Q5. Is `Mutex<T>` always `Sync`?

**Interview Answer**

Yes, `Mutex<T>` is `Sync` when `T: Send`. The mutex ensures only one thread accesses the data at a time, making shared `&Mutex<T>` references safe across threads. However, `MutexGuard<T>` is `Send` but not `Sync`—you can send the guard to another thread but can't share it, which enforces exclusive access.

---

### Q6. Can you manually implement `Sync` for a type?

**Interview Answer**

You can use `unsafe impl Sync for MyType {}` but only if you guarantee that concurrent shared access is safe. This is necessary when wrapping non-`Sync` types in a synchronization primitive. For example, a `OnceCell<T>` wrapping a `RefCell<T>` might need `unsafe impl Sync` if the `OnceCell` provides its own synchronization guarantees.

---

### Q7. What is the `!Sync` marker type?

**Interview Answer**

Rust has `std::marker::PhantomData<*const ()>` to create `!Send` and `!Sync` types, and `PhantomData<*mut ()>` for `!Send`. You can use these in structs to opt out of auto-trait implementation. For example, a handle to a thread-local resource would use `PhantomData<*const ()>` to prevent accidental sharing.

---

### Q8. How does `Sync` relate to lock-free data structures?

**Interview Answer**

Lock-free structures must be `Sync` to allow multiple threads to access them concurrently. The challenge is ensuring that internal invariants hold under concurrent modification. In Rust, `AtomicXxx` types are `Sync`, which is why they're the building blocks for lock-free queues, stacks, and counters that need `Sync` guarantees.

---

### Q9. What is the difference between `Sync` and `Send` in terms of shared vs owned access?

**Interview Answer**

`Send` is about transferring ownership to another thread—it's safe because the original thread no longer has access. `Sync` is about sharing references—multiple threads can read the same data simultaneously. This is why `&mut T` is `Send` (exclusive access transfers safely) but not `Sync` (you can't have multiple mutable references).

---

### Q10. How do `Send` and `Sync` affect trait objects?

**Interview Answer**

Trait objects like `dyn Future + Send + Sync` require the concrete type to implement both traits. When building async services, you often need `Box<dyn Service<Req, Res, Error = E> + Send + Sync>` to store handlers in a collection. If any captured state is `!Send` or `!Sync`, the entire trait object fails to compile.

---
