Generate types from rust codebase.

You can use this library to generate type definitions for other programing languages. For example typescript...

# Example

```rust
use typegen::GetType;

fn ty_str<T: GetType>(_: T) -> String {
    format!("{:?}", T::get_ty())
}
assert_eq!(ty_str(0), "i32");
assert_eq!(ty_str('a'), "char");
assert_eq!(ty_str(true), "bool");
assert_eq!(ty_str(String::new()), "String");
```