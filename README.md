# Rust Websockets

Comparing the default `axum` , `axum-default` and `tungstenite` (with 1,024 sized buffer).

Adding into the comparions, different memory allocators.

We first measure idle, then 10,000 connections, then we disconnect all connections.

Measurments are in MB done via [memory-stats](https://crates.io/crates/memory-stats) crate with `always_use_statm` feature.

## Default

| Example               | Idle     | 10,000 connections  | Diconnect   |
|-----------------------|----------|------------|-------------|
| **axum-example**      | 6.62  | 1334.125 | 45.46    |
| **axum-tws-example**  | 6.62  | 78.0   | 77.86    |
| **tungstenite-example**| 5.37  | 69.5   | 49.88    |

## libc malloc_trim(0)

| Example               | Idle     | 10,000 connections  | Diconnect   |
|-----------------------|----------|------------|-------------|
| **axum-example**      | 6.5  | 1334.5 | 9.98    |
| **axum-tws-example**  | 6.37  | 78.12   | 9.54    |
| **tungstenite-example**| 5.37  | 51.91   | 16.67    |

## mimalloc mi_collect(true)

| Example               | Idle     | 10,000 connections  | Diconnect   |
|-----------------------|----------|------------|-------------|
| **axum-example**      | 7.0  | 1294.12 | 416.96   |
| **axum-tws-example**  | 6.87  | 72.75   | 54.66    |
| **tungstenite-example**| 5.87  | 44.69   | 36.86    |


## jemalloc background_threads

| Example               | Idle     | 10,000 connections  | Diconnect   |
|-----------------------|-----------|------------|-------------|
| **axum-example**      | 9.75  | 1332.25 | 24.26953125    |
| **axum-tws-example**  | 9.375   | 73.73   | 15.55    |
| **tungstenite-example**| 7.87   | 47.24   | 14.19    |


# Summary

* Using `tungstenite` directly isn't a real option with `axum` due to the `API` it requires.
* The defulat `axum` setting uses `128K` buffer which is too big for more cases, making `axum-tws` a better option (tried changing `axum-tws` config but saw no real diferences in results).
* The difference between `axum` and `axum-tws` is over `1.3GB` versus `77MB` , very significant.
* `malloc_trim(0)` seems to be able to reclaim the most memory but `jemalloc` is known to help in memory fragmentation and not far behind in performance, while `mimalloc` is lagging considerably. 


