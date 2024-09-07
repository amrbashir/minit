---
"muda": patch
---

Use `objc2` internally, leading to much better memory safety.

The crate will panic now if used from a thread that is not the main thread.
