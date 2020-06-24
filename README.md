# compile-fail

A simple utility for ensuring that certain code does not compile.

It's often necessary to ensure that certain code does not compile. For instance, 
let's say you want to be sure that a method does not implement send:

```rust
pub struct MyType {
    _ptr: *mut (),
}

impl MyType {
    pub fn new() -> Self {
        Self {
            _ptr: std::ptr::null_mut(),
        }
    }
}
```

To test that trying to send this over a thread boundary fails to compile, you can use the following:

```rust
#[cfg(test)]
mod tests {
    use compile_fail::compile_fail;

    #[compile_fail]
    fn cannot_send_non_send_value() {
        let t = super::MyType::new();
        // `t` is not send so this should not compile
        std::thread::spawn(|| t)
    }
}
```

### Downsides

This crate is probably not appropriate for large scale testing of items that should fail as any compilation failure 
will trigger a passing test. For more robust testing use something like [`trybuild`](https://crates.io/crates/trybuild).

To quickly see that your test does indeed fail to compile, you can easily turn off the compile failure testing like so:

```rust
#[compile_fail(off = true)]
```