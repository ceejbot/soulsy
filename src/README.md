# source layout

The bridge between Rust and C++ is defined in [`lib.rs`](./lib.rs) using [cxx](https://cxx.rs/).

Required SKSE plugin code, hooks into the game engine, and UI rendering code is all in the [`plugin/`](./plugin/) subdirectory. The Rust side is responsible for data management and controller logic, and is all in the [`./controller/`](./controller/) subdirectory.
