## rust-errno

This module lets you inspect the value of `errno`, the magic thread-local value
that glibc and other libraries use to signal errors.

You probably don't need it unless you're dealing with C functions directly, for
example because you're writing Rust bindings to an existing C library.

### how to build

```
rustc --lib errno.rs
```

### how to use

```rust
use errno;
use std;

import strerror = errno::strerror;
import errno = errno::errno;

fn main() {
  std::io::file_reader("/does/not/exist");
  std::io::println(strerror(errno())); // prints "No such file or directory"
}
```

### known bugs

Currently only works on Linux. Requires glibc 2.3.4 or better. Support for OS X
and Solaris may be added in the future.
