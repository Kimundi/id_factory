A library for creating typed or untyped id numbers.

Example:

```rust
use id_factory::untyped::IdFactory;

let mut factory = IdFactory::new();

let id1 = factory.next_id();
let id2 = factory.next_id();
assert!(id1 != id2);
```
