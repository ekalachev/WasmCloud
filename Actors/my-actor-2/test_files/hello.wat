(module
  (func $hello (import "" "hello") (result i32))
  (func (export "run") (result i32) (call $hello))
)
