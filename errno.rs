/*
 * Copyright (c) 2012, Ben Noordhuis <info@bnoordhuis.nl>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

#[license = "ISC"];

#[link(name = "errno",
       vers = "1.0",
       author = "Ben Noordhuis <info@bnoordhuis.nl>")];

use std; // required by the tests

import libc::{size_t, c_int};

#[nolink]
native mod __glibc {
  fn __xpg_strerror_r(errnum: c_int, buf: *libc::c_char, len: size_t) -> c_int;
  fn __errno_location() -> *c_int;
}

fn errno() -> int {
  unsafe { *__glibc::__errno_location() as int }
}

fn strerror(errnum: int) -> str {
  let bufv = vec::from_elem(1024u, 0u8);
  let len = vec::len(bufv);
    
  unsafe {
    let buf : *libc::c_char = ::unsafe::reinterpret_cast(bufv);
    let r =__glibc::__xpg_strerror_r(errnum as c_int, buf, len as size_t);
         
    if (r as bool) {
      fail #fmt("__glibc::__xpg_strerror_r() failed [errno=%d]", errno());
    }

    str::unsafe::from_c_str(buf)
  }
}

#[test]
fn test() {
  // FIXME chokes on localized messages
  assert strerror(22) == "Invalid argument";
  assert strerror(0) == "Success";
}

#[should_fail]
#[test]
fn test2() {
  strerror(-1);
}
