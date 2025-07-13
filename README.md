# Undefined Behavior with Raw Pointers and Vectors in Rust

This repository demonstrates a common mistake in Rust where modifying a vector through a raw pointer leads to undefined behavior if the vector's ownership is changed. The example showcases the issue and provides a safe solution.

## Bug
The `bug.rs` file contains code that attempts to modify a vector using a raw pointer. This is unsafe if the vector is subsequently moved or dropped, resulting in memory corruption or crashes.

## Solution
The `bugSolution.rs` file demonstrates a safe approach using proper memory management techniques.