/*!
A library for creating typed id numbers.

Example:

```
use id_factory::untyped::IdFactory;

let mut factory = IdFactory::new();

let id1 = factory.next_id();
let id2 = factory.next_id();
assert!(id1 != id2);
```
*/

pub mod typed;
pub mod untyped;
