1. **运行 `cargo new my-project`**

   - 创建了 **一个 Rust 包（package）**。
   - 默认包含 **一个二进制 crate**，其 **crate 根是 `src/main.rs`**。

2. **Cargo 约定：**

   - `src/main.rs` → **二进制 crate**。
   - `src/lib.rs` → **库 crate**。
   - `src/bin/*.rs` → **多个二进制 crate**。

3. **一个包（package）可以包含多个 crate**
   - **`src/main.rs` + `src/lib.rs`** → 既有 **二进制 crate**，又有 **库 crate**。
   - **`src/bin/*.rs`** → 拥有多个 **独立的二进制 crate**。
