# epoll, kqueue, and IOCP

## Interview Question

Explain epoll, kqueue, and IOCP and how they relate to async runtimes like Tokio.

## Interview Answer

epoll (Linux), kqueue (macOS/BSD), and IOCP (Windows) are operating system APIs for scalable, event-driven I/O that allow a single thread to monitor thousands of file descriptors efficiently. Tokio's default reactor on Linux uses epoll via the mio crate to detect when sockets become readable or writable, driving the async/await state machine without blocking. On macOS, Tokio uses kqueue, and on Windows it uses IOCP. These APIs are the foundation that makes Rust's async runtime performant — they replace the thread-per-connection model with an event loop that handles thousands of concurrent connections with minimal system resources. Without them, each TCP connection would require its own thread, limiting scalability to a few thousand connections instead of hundreds of thousands.

---

## Follow-up Questions & Answers

### Q1. How does epoll work internally?

**Interview Answer**

epoll uses three system calls: epoll_create to create an epoll instance, epoll_ctl to add, modify, or remove file descriptors from monitoring, and epoll_wait to block until events occur. When a file descriptor becomes ready (readable, writable, or errors), the kernel adds it to a ready list, and epoll_wait returns immediately with the set of ready descriptors. epoll uses an interest list (all monitored FDs) and a ready list (FDs with events), and it supports both level-triggered and edge-triggered modes. Edge-triggered mode, which Tokio uses via mio, notifies only on state transitions rather than whenever data is available, reducing redundant wakeups.

---

### Q2. What is the difference between level-triggered and edge-triggered epoll?

**Interview Answer**

In level-triggered mode, epoll reports events as long as the file descriptor remains in a ready state — if data is available on a socket, epoll will keep notifying on every call to epoll_wait until all data is read. In edge-triggered mode, epoll notifies only when the state changes — it fires once when new data arrives and does not notify again until more data arrives or the condition changes. Edge-triggered is more efficient because it produces fewer spurious wakeups, but it requires reading all available data when notified (typically in a loop) to avoid missing events. Tokio uses edge-triggered mode for optimal performance.

---

### Q3. How does kqueue differ from epoll?

**Interview Answer**

kqueue is the BSD/macOS equivalent of epoll, using kevent() system calls instead of epoll_ctl/epoll_wait. kqueue uses a more general event model based on filters (EVFILT_READ, EVFILT_WRITE, EVFILT_VNODE, etc.) and flags (EV_ADD, EV_DELETE, EV_CLEAR). Unlike epoll, kqueue can monitor not just sockets but also file system changes, signals, and process events through a unified interface. Both achieve the same goal of scalable I/O multiplexing, but their APIs differ. Tokio's mio crate abstracts over both, providing a consistent interface regardless of the underlying OS.

---

### Q4. What is IOCP and how does it differ from epoll/kqueue?

**Interview Answer**

IOCP (I/O Completion Ports) is Windows' asynchronous I/O mechanism that works fundamentally differently from epoll/kqueue. While epoll/kqueue are readiness-based (they notify when a file descriptor is ready for reading/writing), IOCP is completion-based — you submit asynchronous I/O requests, and the OS notifies you when they complete. This means with IOCP, the kernel handles the actual I/O operation rather than just signaling readiness. This can be more efficient for disk I/O but requires a different programming model. Tokio's Windows support uses mio's abstraction, which wraps IOCP into a readiness-based interface for consistency.

---

### Q5. How does Tokio use epoll/kqueue/IOCP internally?

**Interview Answer**

Tokio's reactor is built on the mio crate, which provides a cross-platform abstraction over these OS APIs. When you create a TcpStream or TcpListener in Tokio, mio registers the file descriptor with the OS API (epoll on Linux, kqueue on macOS, IOCP on Windows). When you .await on an async read or write, Tokio registers interest with the reactor and yields the task. When the OS signals that the socket is ready, the reactor wakes the task, and the read/write proceeds. This entire mechanism is hidden behind async/await, but underneath, mio and the OS I/O multiplexing API are what make it work.

---

### Q6. Why can't Rust's async/await work without an OS I/O multiplexing API?

**Interview Answer**

async/await is just syntactic sugar for a state machine — it does not inherently provide I/O operations or task scheduling. Without an OS API like epoll, you would have no way to efficiently wait on thousands of sockets simultaneously. You would need to either block on each socket (thread-per-connection) or busy-loop (wasting CPU). The OS I/O multiplexing API provides the bridge: it tells the runtime which sockets have events, allowing the async runtime to poll only the tasks that are actually ready to make progress. This is what makes async Rust both efficient and correct.

---

### Q7. What are the performance implications of using epoll in a Tokio application?

**Interview Answer**

epoll allows a single Tokio worker thread to handle tens of thousands of concurrent connections because it only wakes up when events actually occur. The overhead per connection is minimal — just a file descriptor and a small amount of kernel memory. In benchmarks, Tokio on Linux can handle over 100,000 concurrent TCP connections with a single thread. The key performance benefit is that the cost scales with active events, not total connections. A million idle connections cost almost nothing in CPU time because epoll_wait returns immediately with only the ready descriptors, avoiding the overhead of checking every connection.

---

### Q8. What is the ABA problem in the context of I/O multiplexing, and does epoll have it?

**Interview Answer**

The ABA problem in I/O multiplexing occurs when a file descriptor number is reused after being closed — if a socket is closed and a new socket is opened, it may get the same file descriptor number. If the epoll interest list is not properly cleaned up, events from the old socket could be delivered to the new socket. epoll handles this by removing closed file descriptors from the interest list automatically (the kernel does this). However, application code must still handle this correctly by deregistering FDs before closing them. mio in Tokio handles this cleanup automatically.

---

### Q9. How does io_uring compare to epoll on modern Linux?

**Interview Answer**

io_uring is a newer Linux I/O interface (introduced in kernel 5.1) that provides asynchronous I/O with a shared ring buffer between user space and kernel space, eliminating most system call overhead. Unlike epoll, which is readiness-based and requires separate read/write syscalls, io_uring supports true async I/O where you submit I/O requests and receive completions without context switching. It can handle both networking and disk I/O. Tokio has experimental io_uring support through the tokio-uring crate, and it offers lower latency and higher throughput than epoll for workloads with many concurrent I/O operations. However, io_uring is still maturing and is not yet the default.

---

### Q10. How do these I/O multiplexing APIs affect the design of a Rust web framework like Axum?

**Interview Answer**

Axum is built on Tokio, which is built on mio, which uses epoll/kqueue/IOCP. This means Axum inherits all the performance characteristics of these OS APIs without needing to think about them directly. Axum handlers can .await on thousands of concurrent requests because Tokio's reactor uses epoll to efficiently monitor all connections. The framework's design — where each request is an async task — maps directly to the event-driven model of epoll. When a handler awaits a database query, the worker thread is free to handle other events from epoll, maximizing throughput. This layered architecture means application code stays simple while the underlying I/O is extremely efficient.
