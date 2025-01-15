# Rust Websockets

Comparing the default `axum` , `axum-default` and `tungstenite` (with 1,024 sized buffer).

Adding into the comparions, different memory allocators.

We first measure idle, then 10,000 connections, then we disconnect all connections.

## Default

| Example               | Idle     | 10,000 connections  | Idle   |
|-----------------------|----------|------------|-------------|
| **axum-example**      | 6946816  | 1398931456 | 47669248    |
| **axum-tws-example**  | 6946816  | 81788928   | 81645568    |
| **tungstenite-example**| 5636096  | 72876032   | 52310016    |

## libc malloc_trim(0)

| Example               | Idle     | 10,000 connections  | Idle   |
|-----------------------|----------|------------|-------------|
| **axum-example**      | 7077888  | 1399324672 | 10469376    |
| **axum-tws-example**  | 6684672  | 81920000   | 10010624    |
| **tungstenite-example**| 5636096  | 54439936   | 17489920    |

## mimalloc mi_collect(true)

| Example               | Idle     | 10,000 connections  | Idle   |
|-----------------------|----------|------------|-------------|
| **axum-example**      | 7340032  | 1356988416 | 437223424   |
| **axum-tws-example**  | 7208960  | 76283904   | 57315328    |
| **tungstenite-example**| 6160384  | 46862336   | 38658048    |


## jemalloc background_threads

| Example               | Idle     | 10,000 connections  | Idle   |
|-----------------------|-----------|------------|-------------|
| **axum-example**      | 10223616  | 1396965376 | 25448448    |
| **axum-tws-example**  | 9830400   | 77320192   | 16314368    |
| **tungstenite-example**| 8257536   | 49541120   | 14880768    |


# Summary

* Using `tungstenite` directly isn't a real option with `axum` due to the `API` it requires.
* The defulat `axum` setting uses `128K` buffer which is too big for more cases, making `axum-tws` a better option (tried changing `axum-tws` config but saw no real diferences in results).
* The difference between `axum` and `axum-tws` is over `1.3GB` versus `77MB` , very significant.
* `malloc_trim(0)` seems to be able to reclaim the most memory but `jemalloc` is known to help in memory fragmentation and not far behind in performance, while `mimalloc` is lagging considerably. 


