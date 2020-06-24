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

#[cfg(test)]
mod tests {
    use compile_fail::compile_fail;

    #[compile_fail]
    fn cannot_add_a_number_and_a_string() {
        12 + "";
    }

    #[compile_fail]
    fn cannot_send_non_send_value() {
        let t = super::MyType::new();
        // `t` is not send so this should not compile
        std::thread::spawn(|| t)
    }
}
