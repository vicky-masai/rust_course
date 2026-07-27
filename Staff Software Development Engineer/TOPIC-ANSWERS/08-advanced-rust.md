# LEVEL 08 — Advanced Rust

---

## Unsafe Rust

### 0205. Unsafe Rust

`unsafe` unlocks operations the compiler can't verify: raw pointers, unsafe functions, mutable statics, unions. You become responsible for invariants. Goal: encapsulate unsafe behind a safe API.

Staff rule: minimize unsafe surface; document invariants; test heavily (Miri when possible).

**Talk track:** *"Unsafe doesn't turn off the borrow checker everywhere — it lets you assert safety the compiler can't prove."*

---

### 0206. Raw Pointers

`*const T` / `*mut T` — no lifetime tracking, can be null/dangling. Needed for FFI, allocators, data structures. Dereferencing is unsafe.

**Talk track:** *"Raw pointers are C-style addresses — power without automatic validity guarantees."*

---

### 0207. Unsafe Functions

Functions marked `unsafe fn` require callers to uphold a contract (validity, alignment, initialization). Document `# Safety` sections.

**Talk track:** *"Unsafe functions push preconditions to the caller — write those preconditions down."*

---

### 0208. Unsafe Traits

Traits where implementing is unsafe because the impl asserts an invariant (`Send`, `Sync` manually, custom marker contracts). Wrong impl = UB everywhere used.

**Talk track:** *"Unsafe traits mean a wrong impl can break the whole program's soundness."*

---

### 0209. Pointer Arithmetic

Offset pointers by bytes/elements. Easy to leave allocated object bounds → UB. Use `offset`/`wrapping_offset` carefully; prefer slices and indexes in safe code.

**Talk track:** *"Pointer math must stay inside allocated objects — leaving is undefined behavior."*

---

### 0210. Manual Memory Management

Allocate/deallocate with allocators, write drops by hand, manage initialization (`MaybeUninit`). Required for custom Vec-like types and FFI buffers.

Easy to leak, double-free, or use uninitialized memory.

**Talk track:** *"Manual memory means you own alloc, init, and free — every path, including errors."*

---

## FFI

### 0211. FFI

Foreign Function Interface — call into (or from) other languages. Bridge Rust's safety with C ABIs. Boundary is where unsafe concentrates.

**Talk track:** *"FFI is the border crossing — validate everything that enters Rust."*

---

### 0212. Rust ↔ C

Most common FFI: `extern "C"` functions, `#[repr(C)]` structs, raw pointers. C strings vs Rust `String` ownership must be clear.

**Talk track:** *"C ABI is the lingua franca — match layout and ownership across the boundary."*

---

### 0213. Rust ↔ C++

Harder: C++ ABI is not stable/portable like C. Usually expose a C wrapper around C++ or use `cxx`/`autocxx` bridges. Exceptions, templates, and overloading don't map cleanly.

**Talk track:** *"Talk to C++ through a C API or a dedicated bridge crate — don't assume ABI compatibility."*

---

### 0214. Rust ↔ Python

PyO3 / rust-cpython embed or extend Python. GIL interactions matter. Great for speeding hot Python paths.

**Talk track:** *"PyO3 lets Rust power Python — respect the GIL and type conversions."*

---

### 0215. extern "C"

ABI specifier for C calling convention — how arguments/returns are passed. Required for interoperable symbols.

**Talk track:** *"extern \"C\" means 'use the C calling convention.'"*

---

### 0216. no_mangle

Stop Rust from name-mangling symbols so C can link to a known function name. Pair with `extern "C"`.

**Talk track:** *"no_mangle exports a stable symbol name for linkers."*

---

## Macros

### 0217. macro_rules!

Declarative macros by example — pattern match syntax and expand. Great for repetitive code, DSLs light. Hygiene rules apply.

**Talk track:** *"macro_rules! is pattern-based code generation — the everyday macro."*

---

### 0218. Procedural Macros

Rust functions that take tokens in and emit tokens out — compile-time plugins. Derive, attribute, function-like forms.

**Talk track:** *"Proc macros are compiler plugins written in Rust that transform code."*

---

### 0219. Derive Macros

`#[derive(Debug, Clone)]` style — generate trait impls from types. Serde's `Serialize` is the classic.

**Talk track:** *"Derives generate boilerplate impls from type structure."*

---

### 0220. Attribute Macros

`#[my_attr]` on items — rewrite functions/modules (e.g., `#[tokio::main]`, tracing instrumentation).

**Talk track:** *"Attribute macros wrap or rewrite whole items."*

---

### 0221. Function-like Macros

`my_macro!(...)` invoked like functions but at compile time (`sqlx::query!`). Can parse custom syntax inside.

**Talk track:** *"Function-like proc macros look like calls but expand at compile time."*

---

### 0222. syn

Crate to parse Rust code into a typed syntax tree inside proc macros. The standard parser for macro authors.

**Talk track:** *"syn turns token streams into structured Rust AST for macros."*

---

### 0223. quote

Crate to emit Rust tokens with interpolation — `quote! { impl #name { ... } }`. Pair with syn.

**Talk track:** *"quote builds token trees to emit generated Rust."*

---

## Serialization

### 0224. Serde

De facto serialization framework — `Serialize`/`Deserialize` traits, derive macros, many formats. Zero-copy-ish patterns exist with care.

**Talk track:** *"Serde is the trait layer; formats are pluggable backends."*

---

### 0225. JSON

Ubiquitous text format for APIs. Human readable; slower/heavier than binary. Schema-less unless you add validation.

**Talk track:** *"JSON is the public API language — convenient, not optimal for internal hot paths."*

---

### 0226. Bincode

Compact binary serialization for Rust-ish data. Fast internal caches/wire formats; not great as a long-term public schema without discipline.

**Talk track:** *"Bincode is compact binary for trusted internal data — version carefully."*

---

### 0227. Protobuf

Schema-first binary format with codegen, evolution rules (field numbers). Excellent for service contracts and gRPC.

**Talk track:** *"Protobuf gives typed, evolvable binary contracts across languages."*

---

### 0228. FlatBuffers

Zero-copy deserialization — access fields without parsing whole message into objects. Great for huge messages / game/net performance.

Harder schema ergonomics than Protobuf sometimes.

**Talk track:** *"FlatBuffers trade convenience for zero-copy access to large messages."*
