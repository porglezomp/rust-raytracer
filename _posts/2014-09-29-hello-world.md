---
layout: post
title: Hello, World
categories:
---

I started work on this ray-tracer in order to try to learn [Rust](http://rust-lang.org), and because I haven't ever made a good ray-tracer before. I started work on this blog in order to document the process I followed to produce this ray-tracer, and hope that it may prove useful to someone else trying to learn rust.

Before we set out upon our journey to produce a ray-tracer, you should first follow [The Guide](http://doc.rust-lang.org/guide.html) if you haven't already, in order to get the compiler and package manager installed.

Done that? Now we can begin.

As with every language, you must start off with a Hello World program. Put the following in `hello.rs`.

```rust
fn main() {
    println!("Hello, World!");
}
```

You can compile it by writing `rustc hello.rs`, and then you can run the executable. You should get the output `Hello, World!` on your console.
Now, with the formalities out of the way, let's begin.
