# Fix DI Write-Lock Deadlocks in Rustium Framework

## Problem
Requests that inject services are deadlocking and timing out. The root cause is a combination of three interacting issues:

1. **Wrong service lifetimes**: `RustiumSettings` and `SurrealDAL` are registered as `scoped()`, causing their constructors to run on EVERY incoming request — including expensive DB connection setup.
2. **`async_std::task::block_on` blocking tokio worker threads**: Both `SurrealDAL::new()` (`lib/src/datastore/surreal_dal.rs:41`) and `RustiumSettings::new()` (`lib/src/settings/mod.rs:43`) call `async_std::task::block_on()` to run async init synchronously. This blocks the current tokio worker thread. Under concurrent load, all workers get blocked waiting for DB connections, starving the runtime.
3. **`spin::RwLock` contention in `more-di` internals**: The `more-di` crate uses `spin::RwLock` (a spinlock) to guard its internal service storage. When a scoped service is being created (write lock held), and the constructor blocks the thread via `block_on`, other threads spin-wait on the lock, burning CPU without progress. Combined with blocked tokio workers, this causes timeouts.

### How the deadlock manifests
1. Request arrives → handler extracts `Inject<dyn IUserService>`
2. DI container acquires internal write lock to create `SurrealDAL` for this scope
3. `SurrealDAL::new()` calls `async_std::task::block_on(this.init())` which establishes a WebSocket connection to SurrealDB — blocking the tokio worker thread while holding the DI write lock
4. Concurrent requests spin-wait on the DI lock, burning CPU, blocking additional workers
5. With all workers blocked/spinning, no async progress is possible → timeout

## Current State
- `example/src/main.rs` registers all services as `.scoped()` (lines 24-34)
- `lib/src/datastore/surreal_dal.rs` — `SurrealDAL::new()` uses `async_std::task::block_on` (line 41) to init DB connection
- `lib/src/settings/mod.rs` — `RustiumSettings::new()` uses `async_std::task::block_on` (line 43) to load config
- `lib/Cargo.toml` depends on `async-std = "1.13.0"` (line 43)
- `surrealdb` uses tokio internally — running its futures on an `async_std` executor is a cross-runtime conflict

## Proposed Changes

### 1. Change service lifetimes: `scoped()` → `singleton()` for infrastructure services
In `example/src/main.rs`, change `RustiumSettings` and `SurrealDAL` from `.scoped()` to `.singleton()`. These are shared infrastructure — settings don't change per-request, and `Surreal<Client>` is designed to be shared across tasks (it multiplexes internally over WebSocket).

`UserRepository`, `UserService`, and `AuthService` can remain `scoped()` since they are lightweight (just hold `Arc` refs to singletons) and may hold per-request state in the future.

This eliminates the per-request DB connection overhead and the write-lock contention under load.

### 2. Replace `async_std::task::block_on` with tokio-compatible blocking
In both `SurrealDAL::new()` and `RustiumSettings::new()`, replace:
```rust
async_std::task::block_on(this.init())
```
with:
```rust
tokio::task::block_in_place(|| {
    tokio::runtime::Handle::current().block_on(this.init())
})
```

`block_in_place` tells the tokio runtime that the current worker is about to block, allowing it to move other tasks to different workers. `Handle::current().block_on()` runs the future on the existing tokio runtime so that `surrealdb`'s tokio-based I/O works correctly.

### 3. Remove `async-std` dependency
Remove `async-std` from `lib/Cargo.toml` and replace all `use async_std::task` imports with the tokio equivalents. This eliminates the cross-runtime issue entirely.

### Summary of file changes
- `lib/Cargo.toml` — remove `async-std` dependency
- `lib/src/settings/mod.rs` — replace `async_std::task::block_on` with tokio `block_in_place`/`Handle::current().block_on()`; remove `async_std` import
- `lib/src/datastore/surreal_dal.rs` — same replacement
- `example/src/main.rs` — change `RustiumSettings::scoped()` and `SurrealDAL::scoped()` to `.singleton()`
