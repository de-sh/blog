---
title: 'Async with rust, a collection of learnings'
slug: 'async-thoughts'
summary: 'A collection of thoughts and links that I feel have had a disproportionate impact on my journey learning async-rust'
date: 'October 15, 2021'
---

I wrote my first few lines of async rust at the end of 2020, that was the first time I was actually learning about the core principles behind async/await, while building a [toy database](https://github.com/de-sh/kvdb/commit/30f428c5263f13f6ebc3b3303d636f0c3c178ee2). Even though I have had prior experience writing code with similar keywords in javascript and python, they didn't make much sense then, it was just what I had to write to get through implementing something in code. Having been pulled into the async world while writing network facing code, I found it to be a learning that also transfered well into college as we were learning about similar stuff in a class on Programming Paradigmns.

The journey into async-rust started for me with some exposure through a well written [tutorial on the tokio project's website](https://tokio.rs/tokio/tutorial), which introduced the why in such a way that async/await totally made sense. This piece is a recollection of material that I've used to make sense of and interpret the crazy complex world of async/await over the past year or so. Anyways, if you are new and only getting started with rust and want to learn about the basics, I'd point you to [this awesome screencast on YouTube by Jon Gjengset(@jonhoo)](https://www.youtube.com/watch?v=ThjvMReOXYM), if you'd still like to read on and learn from my journey, it's my pleasure :D

Writing a lock and load mechanism for handling shared memory between threads was interesting, even more interesting was learning about how `lock()` in `std` differed from `lock()` in `tokio`. The way in which these two handled I/O also made for an interesting comparison. How all of tokio managed well with a `.await`, which until now had to be a `io::Result<_>` in std.

To give a brief, async-rust code looks like this:
``` rust
async fn hello() -> String {
  // Code that waits on some IO
}

fn main() {
  let runtime = runtime_builder();                          // Creates a runtime for async operations
  print!("{}", runtime.on_block(async { hello().await }));  // Calls the function from within built runtime and blocks main() till it returns
}
```

What happens here is simple, we have an async function `hello()` running from within a runtime that blocks `main()`. Wait, so what's an async function and how is it different from the regular function? Well, they are similar in how they are written, but differ in that an async function starts with the keywords `async fn` unlike the normal `fn`, here `async` is syntactic sugar that translates down as depicted below:
```rust
async fn hello() -> String {
  // Logic
}
```
```rust
fn hello() -> Future<Output = String> {
  // Logic
}
```

till the "future" returns, in which case the value is then printed to screen by `print!()`. There's a lot of thought that has gone into how async/await within runtimes is a lot better than callbacks that have to be handled manually(as was once the norm, *casually points to javascript's `.then()`*), a debate I am not going to expand upon. Here's take where this problem is presented in a classic blog called ["What color is your function"](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/), a well articulated read if I am to recommend one, in which async functions and regular functions are called colored functions that allow one to call the other, but not vice versa and how things evolved till it got here.

Another great feature of async code that I've come to use a lot is `select!`, an async construct that performs multiplexing of async tasks. It has been very helpful as far as writing concurrent, multi-threaded IO/Network heavy applications goes, a note to read about and understand this is one [from the tokio tutorial](https://tokio.rs/tokio/tutorial/select).

There's a lot more complex, but fun stuff that happens here, and a lot of this is abstracted away to make the experience a breeze for beginners, but it's still well worth the time to delve into the guts of this exciting coding paradigm. I for one, will be continuing to learn a lot, that's for sure, until the next one, farewell friends :D