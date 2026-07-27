# LEVEL 03 — Rust Language

---

## Ownership & Memory

### 0105. Ownership

Rust's core rule: each value has one owner; when the owner goes out of scope, the value is dropped (freed). Assignment moves ownership by default for non-`Copy` types.

This is how Rust gets memory safety without a GC — the compiler tracks who is responsible for cleanup.

**Talk track:** *"Ownership means exactly one responsible party for freeing a value — moves transfer that responsibility."*

---

### 0106. Borrow Checker

The compiler rules that enforce: either many immutable borrows XOR one mutable borrow, and borrows can't outlive the owner. Prevents data races and use-after-free at compile time.

Fighting the borrow checker usually means your mental model of lifetimes/aliases is unclear — not that Rust is being petty.

**Talk track:** *"The borrow checker is aliasing + lifetime rules as a type system. It encodes 'who can read/write this now'."*

---

### 0107. References

`&T` shared reference, `&mut T` exclusive mutable reference. References borrow; they don't own. They must always be valid (no null references in safe Rust).

Passing references lets functions use data without taking ownership.

**Talk track:** *"References are temporary loans — shared or exclusive — checked by the compiler."*

---

### 0108. Move Semantics

Reading a non-`Copy` value moves it; the old binding can't be used. Prevents double free. Return values move out; you can partially move out of structs carefully.

`Copy` types are duplicated instead (integers, bools, small flags).

**Talk track:** *"Moves transfer ownership byte-for-byte conceptually; the source becomes unusable so nothing is freed twice."*

---

### 0109. Copy Trait

Marker trait: type can be duplicated by bitwise copy. Implicitly copied on assign. Must be `Clone` too. No custom destructor logic (`Drop` forbidden with `Copy`).

Use for small, plain values. Don't implement for types managing resources.

**Talk track:** *"Copy means 'duplicate is free and identical' — fine for u64, wrong for Vec."*

---

### 0110. Clone Trait

Explicit deep (or logical) duplication via `.clone()`. Can be expensive. Derive when fields are `Clone`.

Staff habit: cloning in hot paths is a smell — share with `Arc` or restructure.

**Talk track:** *"Clone is intentional duplication. Ask whether sharing would be better."*

---

### 0111. Drop Trait

Custom cleanup when a value goes out of scope — free heap, close FDs, unlock (careful), decrement refcounts. RAII pattern: acquire in constructor, release in `Drop`.

Don't panic in drop if avoidable; beware double-panic aborts.

**Talk track:** *"Drop is Rust's destructor — RAII cleanup without a GC pause."*

---

### 0112. Drop Order

Struct fields drop in declaration order (reverse of construction typically for locals — locals drop reverse order of declaration). Important when field A must outlive field B during teardown (e.g., references inside self — usually forbidden in safe code; more relevant with unsafe / pinning).

Knowing order prevents subtle use-after-free in unsafe and helps reason about locks/guards.

**Talk track:** *"Drop order is deterministic. When cleanup order matters, make it explicit in types."*

---

### 0113. Deref

`Deref`/`DerefMut` let smart pointers behave like their inner type (`Box<T>` acts like `T` via auto-deref in method calls). `*` operator uses `Deref`.

Don't overuse as "inheritance" — it's for pointer-like types.

**Talk track:** *"Deref is how Box and friends feel transparent without being the inner type."*

---

### 0114. Deref Coercion

Compiler inserts `&` / `*` coercions along `Deref` chains: `&String` → `&str`, `&Vec<T>` → `&[T]`. Ergonomic APIs accept `&str` even if callers have `String`.

**Talk track:** *"Deref coercion is why APIs take &str and still accept &String smoothly."*

---

### 0115. Interior Mutability

Mutate through a shared reference using runtime (or unsafe) checks: `Cell`, `RefCell`, `Mutex`, `RwLock`, atomics. Safe Rust's "shared XOR mutable" becomes "checked at runtime" for these wrappers.

Needed for graphs, caches, mock counters, shared config behind `Arc`.

**Talk track:** *"Interior mutability relocates the borrow rule from compile time to runtime (or atomics)."*

---

### 0116. Memory Layout

How types sit in memory: size, align, field order, niches (`Option<&T>` same size as `&T`). `#[repr(C)]` for FFI stability. Enum discriminants and packing matter for perf and FFI.

**Talk track:** *"Layout is the ABI of your types. Niche optimization is why Option of a reference is still one pointer."*

---

### 0117. Zero Cost Abstractions

Rust's promise: high-level abstractions (iterators, generics) compile down to what you'd write by hand — no mandatory runtime tax. You pay only for what you use.

Verify with benchmarks/`cargo asm` when it matters; the promise holds when LLVM can see through generics (monomorphization).

**Talk track:** *"Zero-cost means abstraction without mandatory overhead — iterators can be as fast as loops."*

---

### 0118. Pinning

`Pin` guarantees a value won't move in memory — required for self-referential types and async state machines that hold pointers into themselves. `Pin<&mut T>` restricts moving out.

You rarely pin by hand in app code; async and some futures internals depend on it.

**Talk track:** *"Pinning freezes an object's address so self-references stay valid — async's dirty secret."*

---

### 0119. Self Referential Types

Structs that point into their own fields. Extremely hard in safe Rust because moves would invalidate pointers. Needs pinning + unsafe or redesign (indices into a Vec instead of pointers).

Prefer redesign over self-ref when possible.

**Talk track:** *"Self-references fight moves. Use indices or Pin+unsafe; prefer redesign."*

---

### 0120. PhantomData

Zero-sized marker telling the compiler about ownership/lifetime/type relationships the type system can't see from fields alone. Used in `Vec`-like types, lifetime branding, FFI wrappers.

Doesn't exist at runtime; affects drop check and variance.

**Talk track:** *"PhantomData is a ghost field for the type checker — encodes ownership without storing data."*

---

## Lifetimes

### 0121. Lifetime Elision

Common patterns where the compiler infers lifetime annotations so you don't write them (`fn foo(&self) -> &str`). Rules are finite — when they don't apply, you write explicit lifetimes.

**Talk track:** *"Elision covers the usual cases; explicit lifetimes appear when the relationship isn't obvious."*

---

### 0122. Explicit Lifetimes

`'a` annotations link references: output borrows from which input. They don't "create" life — they describe relationships the compiler must verify.

**Talk track:** *"Lifetimes are relationships between borrows, not stopwatches."*

---

### 0123. Generic Lifetimes

Functions/structs parameterized by lifetimes: `struct Ref<'a, T> { r: &'a T }`. The struct can't outlive the data it references.

**Talk track:** *"Generic lifetimes parameterize 'how long this borrow is valid' into the type."*

---

### 0124. Lifetime Bounds

`T: 'a` means all references in `T` live at least `'a`. Needed when storing generic values that might contain borrows.

**Talk track:** *"T: 'a says T doesn't contain short-lived references shorter than 'a."*

---

### 0125. `'static` Lifetime

Lives for the entire program: string literals, global data, or owned data with no borrows (`String` can be `'static` as a type bound meaning "owns its data"). Often misunderstood — `'static` bound ≠ must be a literal.

**Talk track:** *"'static means valid forever — either a literal/global or fully owned with no short borrows."*

---

### 0126. Higher Ranked Trait Bounds (HRTB)

`for<'a> ...` — works for all lifetimes. Shows up with closures and `Fn` traits that borrow with any short lifetime. Advanced but appears in real APIs (`serde`, thread scopes historically).

**Talk track:** *"HRTB says 'this works for every lifetime,' not just one named 'a."*

---

### 0127. Lifetime Variance

Whether you can substitute longer/shorter lifetimes (`&'static` where `&'a` expected — covariance). `&mut` is invariant in `T`. Wrong variance = unsound APIs; PhantomData chooses variance.

**Talk track:** *"Variance decides when longer-lived references can stand in for shorter ones — critical for unsafe API design."*

---

### 0128. Lifetime Subtyping

`'long: 'short` means `'long` outlives `'short`. A reference with a longer lifetime can coerce to a shorter one (covariant positions).

**Talk track:** *"Longer lifetimes are subtypes of shorter ones where coercion is allowed."*

---

### 0129. Async Lifetimes

Async functions capture borrows across `.await` points — the future must stay valid while suspended. Often forces `'static` bounds on spawned tasks (`tokio::spawn`), pushing you to own data (`Arc`, clone) instead of borrowing.

**Talk track:** *"Awaiting holds borrows across suspend points — spawn usually wants owned/'static data."*

---

## Traits & Generics

### 0130. Trait Bounds

`T: Display + Clone` constrains generics so you can call those methods. Bounds are the API contract for generic code.

**Talk track:** *"Bounds declare what capabilities a generic type must have."*

---

### 0131. Associated Types

Types tied to a trait (`Iterator::Item`). One concrete type per implementor — simpler than extra generic params when there's a natural single choice.

**Talk track:** *"Associated types name an output type that belongs to the implementation."*

---

### 0132. Generic Associated Types (GAT)

Associated types that are themselves generic (`type Item<'a>`). Enables streaming iterators and advanced patterns. Stabilized later in Rust's life — appears in async traits / advanced libraries.

**Talk track:** *"GATs let associated types take lifetime/type parameters — richer trait designs."*

---

### 0133. Trait Objects

`dyn Trait` — type erasure, vtable dispatch. Heterogeneous collections (`Vec<Box<dyn Error>>`). Requires object safety.

Tradeoff: flexibility vs monomorphized speed; fatter pointers (data + vtable).

**Talk track:** *"Trait objects erase the concrete type for runtime polymorphism."*

---

### 0134. Static Dispatch

Generics monomorphize — each concrete type gets its own compiled copy. Fast, inlinable; can increase binary size.

Default for most Rust APIs.

**Talk track:** *"Static dispatch generates specialized code per type — speed at the cost of code size."*

---

### 0135. Dynamic Dispatch

Call through vtable (`dyn Trait`). One copy of code; indirect call. Useful for plugins and mixed types.

**Talk track:** *"Dynamic dispatch shares code and decides the method at runtime via a vtable."*

---

### 0136. Send

Marker: type can be transferred across thread boundaries. `Rc` is not `Send`; `Arc` is. Required for `std::thread::spawn`.

**Talk track:** *"Send means ownership can move to another thread safely."*

---

### 0137. Sync

Marker: `&T` is safely shareable across threads (`T: Sync` ≈ `&T: Send`). `Mutex<T>` is `Sync` if `T: Send`. `Cell` is not `Sync`.

**Talk track:** *"Sync means shared references are thread-safe — interior mutability must handle concurrency."*

---

### 0138. Sized

Types with known size at compile time. Most generics default to `T: Sized`. `?Sized` allows unsized types (`str`, `[T]`, `dyn Trait`) behind pointers.

**Talk track:** *"Sized is the default; ?Sized unlocks slices and trait objects as generic params behind fat pointers."*

---

### 0139. Marker Traits

Traits with no methods that encode properties: `Copy`, `Send`, `Sync`, `Unpin`. Pure type-system information.

**Talk track:** *"Marker traits are compile-time labels for safety properties."*

---

### 0140. Auto Traits

Traits automatically implemented when fields allow (`Send`, `Sync`, `Unpin`). Unsafe to wrongly implement/opt out implications.

**Talk track:** *"Auto traits propagate through your struct based on field types."*

---

### 0141. Blanket Implementations

`impl<T: Foo> Bar for T` — implement for every type matching a bound. Powers much of the standard library's coherence (`From`/`Into`).

**Talk track:** *"Blanket impls give traits to whole families of types at once."*

---

### 0142. Monomorphization

Compiler stamps out concrete versions of generic functions per type used. Enables static dispatch and inlining; can bloat binaries if overused with many types.

**Talk track:** *"Monomorphization is compile-time specialization of generics — the engine behind zero-cost generics."*
