# Rust Memory Management

## How does Rust memory management work?

Rust manages memory using **ownership** — a set of rules the compiler checks at build time. Every value in your program has exactly **one owner** (one variable). When you assign a value to another variable or pass it to a function, ownership **moves** to the new place and the old variable becomes invalid. If you only need to read or modify a value without taking it, you **borrow** it using `&T` (read-only) or `&mut T` (write-only). When the owner goes out of scope — at the closing `}` — Rust automatically calls `drop()` and frees the memory. There is no garbage collector and no manual `free()`. All checks happen at compile time, so you get C-level speed with memory safety built in.

---

## Stack vs Heap

| | **Stack** | **Heap** |
|---|-----------|----------|
| **Speed** | Very fast | Slower |
| **Size** | Fixed at compile time | Can grow at runtime |
| **Data examples** | `i32`, `bool`, `char` | `String`, `Vec<T>`, `Box<T>` |
| **Who frees it?** | Automatic when scope ends | Automatic when owner is dropped |
| **Cost to create** | Almost free | Needs memory allocation |

**Simple rule:** Small fixed data → stack. Big or growable data → heap.

---

## How Stack and Heap Connect

A `String` is not one block of memory. It is two parts: a small **handle** on the stack, and the **actual text** on the heap.

```text
 STACK (24 bytes on 64-bit)              HEAP (variable size)
 ┌──────────┬────────┬──────────┐         ┌───┬───┬───┬───┬───┐
 │ ptr ─────┼────────┼──────────┼──┬──►  │ h │ e │ l │ l │ o │
 │ len: 5   │        │ cap: 5   │  │     └───┴───┴───┴───┴───┘
 └──────────┴────────┴──────────┘  │
                                   │
                          stack handle points
                          to heap address
```

- **Stack part:** pointer + length + capacity. Always 24 bytes.
- **Heap part:** the real character data. Size changes at runtime.
- **`Vec<T>`** works the same way.

```rust
fn main() {
    let x: i32 = 42;                        // all on stack — 4 bytes
    let s: String = String::from("hello");    // 24-byte handle on stack + bytes on heap

    println!("stack: {x}, heap: {s}");
}
```

---

## Ownership — 3 Rules

These three rules control all memory in safe Rust.

| # | Rule |
|---|------|
| **1** | Every value has **one owner** (one variable). |
| **2** | There can be **only one owner** at a time. |
| **3** | When the owner goes out of scope (`}`), the value is **dropped** — memory is freed. |

**Real-world meaning:** Think of a house key. Only one person should hold the key that can sell the house. When that person leaves, the house gets cleaned up automatically.

```rust
fn main() {
    let s = String::from("Rust");  // s owns the heap data
    println!("{s}");               // s is still alive here
} // <-- s goes out of scope HERE
  //     drop() runs automatically
  //     heap memory is freed
```

You never call `free()`. Rust inserts the cleanup at the closing `}`.

---

## Move Semantics

When you assign a heap type to another variable, Rust **moves** ownership. It copies the 24-byte handle. The heap data does not move.

The old variable becomes **DEAD**. The compiler blocks any use of it.

```text
 BEFORE:  s1 is owner
 ┌──────────────┐         ┌───┬───┬───┬───┬───┐
 │ s1: ptr,len  │────────►│ R │ u │ s │ t │   │
 └──────────────┘         └───┴───┴───┴───┴───┘

 AFTER: let s2 = s1;
 ┌──────────────┐         ┌───┬───┬───┬───┬───┐
 │ s2: ptr,len  │────────►│ R │ u │ s │ t │   │  <-- same heap data
 └──────────────┘         └───┴───┴───┴───┴───┘

 s1: DEAD / INVALID  <-- compiler error if you use s1
 s2: ONLY OWNER      <-- s2 must free the heap at scope end
```

**Why s1 must die:** If both s1 and s2 were alive, both would try to free the same heap block. That is a **double free** — a serious bug. Rust stops this at compile time.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves: s1 --> s2

    // println!("{s1}"); // DEAD — compile error E0382
    println!("{s2}");    // s2 is the only valid owner
} // s2 dropped here --> heap freed once
```

### Move into a function

Passing a value to a function also moves ownership.

```rust
fn process(data: String) {
    println!("{data}");
} // data dropped here --> heap freed inside function

fn main() {
    let msg = String::from("hello");
    process(msg);         // msg moves into function
    // println!("{msg}"); // DEAD — msg was moved
}
```

### Move vs Copy vs Clone

| Action | What happens | Old variable |
|--------|-------------|--------------|
| **Move** | Handle copied to new owner | DEAD |
| **Copy** | Full bitwise copy (stack types only) | Still alive |
| **Clone** | Deep copy of heap data | Still alive |

```rust
fn main() {
    // Copy — works for i32, bool, char (no heap data)
    let a: i32 = 10;
    let b = a;              // copy, not move
    println!("{a} {b}");    // both alive

    // Clone — explicit deep copy for heap types
    let v1 = String::from("hi");
    let v2 = v1.clone();    // new heap buffer created
    println!("{v1} {v2}");  // both alive, separate heap data
}
```

---

## Borrowing

You do not always need to move ownership. You can **borrow** — give temporary access without giving up ownership.

### Immutable borrow: `&T`

- Read-only access.
- Owner keeps the value.
- **Many** `&T` borrows allowed at the same time.

```rust
fn print_len(s: &String) {       // borrow — read only
    println!("len = {}", s.len());
} // borrow ends here — nothing is dropped

fn main() {
    let msg = String::from("hello");
    print_len(&msg);             // create &T reference
    println!("{msg}");           // msg still alive — still owner
}
```

### Mutable borrow: `&mut T`

- Read and write access.
- Owner keeps the value.
- **Only one** `&mut T` allowed at a time.

```rust
fn add_exclaim(s: &mut String) { // borrow — read + write
    s.push('!');
} // borrow ends here

fn main() {
    let mut msg = String::from("hello");
    add_exclaim(&mut msg);       // create &mut T reference
    println!("{msg}");           // msg still alive — still owner
}
```

### The Golden Rule

> At any moment, you can have **either**:
> - **Many** `&T` (shared read), **OR**
> - **One** `&mut T` (exclusive write)
>
> **Never both. Never two `&mut T`.**

| Allowed? | Situation |
|----------|-----------|
| Yes | 5 x `&T` at the same time |
| Yes | 1 x `&mut T` alone |
| No | `&T` + `&mut T` together |
| No | 2 x `&mut T` together |

**Why:** Multiple readers do not conflict. But a writer and a reader (or two writers) at the same time causes data corruption. Rust blocks this at compile time.

```rust
fn main() {
    let mut s = String::from("hi");

    let r1 = &s;          // OK — immutable borrow
    let r2 = &s;          // OK — another immutable borrow
    println!("{r1} {r2}");

    let r3 = &mut s;      // OK — no other borrows active now
    r3.push('!');
    println!("{r3}");
}
```

```rust
// COMPILE ERROR — Golden Rule broken
fn main() {
    let mut s = String::from("hi");
    let r1 = &s;          // immutable borrow active
    let r2 = &mut s;      // ERROR — cannot borrow mutably while immutably borrowed
    println!("{r1}");
}
```

### References must not outlive the owner

A borrow cannot live longer than the data it points to.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
    // returned reference lives only as long as x and y
}

fn main() {
    let s1 = String::from("long string");
    let s2 = String::from("short");
    let result = longest(s1.as_str(), s2.as_str());
    println!("{result}"); // OK — s1 and s2 still alive
}
```

---

## Drop — Automatic Cleanup

When a value goes out of scope, Rust calls `drop()` automatically. This frees heap memory and closes files, sockets, and locks.

```text
 create value --> use value --> scope ends (}) --> drop() --> memory freed
```

```rust
struct Log(&'static str);

impl Drop for Log {
    fn drop(&mut self) {
        println!("freeing: {}", self.0);
    }
}

fn main() {
    let outer = Log("outer");
    {
        let inner = Log("inner");
    } // inner dropped first --> prints "freeing: inner"
} // outer dropped last --> prints "freeing: outer"
```

---

## Rust vs Other Languages

| | **Rust** | **C / C++** | **Java / Python / Go** |
|---|----------|-------------|------------------------|
| **Strategy** | Compiler tracks ownership at build time | Programmer calls `free()` / `delete` manually | Garbage collector scans and frees unused objects |
| **Speed** | No runtime overhead — same as C | Fast — no GC | Slower — GC pauses and extra CPU |
| **Memory safety** | Guaranteed in safe code at compile time | Not guaranteed — bugs at runtime | Safe, but GC can pause your program |
| **When memory is freed** | Exactly at scope end — predictable | When programmer remembers | When GC decides — unpredictable |
| **Main risk** | Learning curve | Use-after-free, double-free, leaks | GC latency, higher memory use |
| **Manual cleanup needed?** | No | Yes | No |

**Bottom line:**

- **C/C++** — fast, but you are responsible for every free. Easy to make dangerous mistakes.
- **Java/Python/Go** — safe, but you pay with speed and unpredictable GC pauses.
- **Rust** — fast like C, safe like a GC language, with zero runtime cost. The compiler does the safety work before your program runs.

---

## Quick Reference

| Concept | Plain English |
|---------|---------------|
| **Stack** | Fast storage for small fixed-size data |
| **Heap** | Flexible storage for growable data |
| **Ownership** | One variable owns each value |
| **Move** | Transfer ownership — old variable is DEAD |
| **Borrow `&T`** | Temporary read access — owner stays alive |
| **Borrow `&mut T`** | Temporary write access — only one at a time |
| **Drop** | Automatic cleanup when scope ends |
| **Clone** | Explicit deep copy — use only when you need two owners |

### The 3 Ownership Rules

1. Every value has **one owner**.
2. Only **one owner** at a time.
3. Owner hits `}` → value is **dropped** → memory is freed.

### Complete Example

```rust
fn read(s: &String)  { println!("read:  len = {}", s.len()); }
fn write(s: &mut String) { s.push('!'); }

fn main() {
    let n: i32 = 42;                          // stack
    let mut msg = String::from("Rust");       // stack handle + heap data

    read(&msg);                               // borrow — msg still owner
    write(&mut msg);                          // mutable borrow — msg still owner

    let copy = msg.clone();                   // deep copy — two separate owners
    println!("{n} | {msg} | {copy}");

    // let moved = msg;                       // move — msg becomes DEAD
    // println!("{msg}");                     // ERROR E0382
}
```
