# wrap_result

## Examples

```rust
use wrap_result::*;
let x = 0;
assert_eq!(x.wrap_ok(), Result::<u32,u32>::Ok(x));

assert_eq!(x.wrap_err(), Result::<u32,u32>::Err(x));

assert_eq!(x.wrap_some(), Some(x));
```
