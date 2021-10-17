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

Multi-threaded apps allow you to better utilise the large number of threads that are now common in newer CPUs, writing these apps on the other hand is a tough ask. With rust and async though, things are significantly better. For one, you can spawn a thread with [`std::thread::spawn()`](https://doc.rust-lang.org/std/thread/fn.spawn.html) and have part of your application run on a dedicated thread. For example, a dedicated thread is a great way to handle Network and IO interfaces separately from the business logic of the app, an example of code written like this would be:
```rust
fn main() {
  std::thread::spawn(|| {
    // Code that is to be run on a separate thread
  });

  // Code to be run on the current(main) thread. e.g. loop {}
}
```

It is to be noted that there should be code that continues to run in the main thread for the code spawned into thread to interact with the terminal, else the app just returns. Thus the difference between the following two code blocks is pretty big:
```rust
fn main() {
    std::thread::spawn(|| {
        for i in 1..100 {
            println!("{}", i);
        }
    });
}
```
Output is empty:
```bash
$ ./run

```
As observed, above code returns as soon as the thread is spawned and doesn't print to the terminal, whereas the following does print the numbers 1 through 99.
```rust
fn main() {
    std::thread::spawn(|| {
        for i in 1..100 {
            println!("{}", i);
        }
    });
    
    loop {}
}
```
Output contains numbers 1 through 99 in order and blocks the terminal afterwards, due to the `loop {}` statement:
```bash
$ ./run
1
2
3
...
99
```

Similarly, if you are constrained by the number of threads you can spawn(depending on the OS), an intelligent way to handle this problem is with [`tokio::task::spawn()`](https://docs.rs/tokio/1.12.0/tokio/task/fn.spawn.html) which has similar syntax, but supports async program scopes that are handled by the runtime(in this case tokio) and can be scheduled to run on the same thread or a different one by using the concept of **green-threads**. Hence when you write the following with `task`s instead of `thread`s, the difference is pretty easy to determine. This in code looks something like:
```rust
#[tokio::main]
async fn main() {
  tokio::task::spawn(async {
    // Code that is to be handled by a concurrent task
  });

  // Rest of main()
}
```
Since tasks are cheaper than full blown threads due to the lesser number of system calls made, they are better for a lot of things and hence are more often used as far as my code goes.

A question that might have now made it's way into your mind right now would be: "If I have multiple threads/tasks handling various facets of my apps operations, how do I ensure that they are working in tandem and not cause chaos?", fret not, the answer I most frequently relate with is the humble channel. Channels are shared memory regions that are shared between multiple threads, one type of channel is the multi-producer-single-consumer [`std::sync::mpsc::channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) which allows for multiple producer thread/tasks to send data into a single consumer thread/task. The above mentioned function returns a tuple of `(Sender<T>, Receiver<T>)` where the receiver can be passed on to the consumer operation and the sender to the producer operations. An example where these could be used is as such:
```rust
fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for i in 1..100 {
            tx.send(i);
        }
    });

    while let Ok(i) = rx.recv() {
        println!("{}", i);
    }
}
```
Output contains 1 to 99 in order, just as the previous example, but here we are printing from `main()` instead of doing it from a spawned thread. There's a possibility that not all numbers will be printed, but the app returns after printing 99 for sure:
```bash
$ ./run
1
2
3
...
99
```
Since multiple producers can exist, the code could also be:
```rust
fn main() {
    // Two channels are necessary to communicate with the two tasks.
    let (tx_a, rx_a) = std::sync::mpsc::channel();
    let tx_b = tx_a.clone();
    std::thread::spawn(move || {
        for i in 1..50 {
            tx_a.send(i);
        }
    });

    std::thread::spawn(move || {
        for i in 50..100 {
            tx_b.send(i);
        }
    });

    while let Ok(i) = rx_a.recv() {
        println!("{}", i);
    }
}
```
Output contains numbers 1 to 49 and 50 to 99 in random order, depending on when they were sent onto the channel, depicting the concurrent nature of processes running on threads:
```bash
$ ./run
50
1
2
51
3
...
```

Another great feature of async code that I've come to use a lot is `select!`, an async construct that performs multiplexing of async tasks. It has been very helpful as far as writing concurrent, multi-threaded IO/Network heavy applications goes, a note to read about and understand this is [from the tokio tutorial](https://tokio.rs/tokio/tutorial/select). A great way to illustrate the use of `select!` is by making the following changes to the last example:
```rust
use tokio::{select, sync::mpsc, task};

#[tokio::main]
async fn main() {
    // Two channels are necessary to communicate with the two tasks.
    let (tx_a, mut rx_a) = mpsc::channel(100);
    let (tx_b, mut rx_b) = mpsc::channel(100);
    task::spawn(async move {
        for i in 1..50 {
            tx_a.send(i).await;
        }
    });

    task::spawn(async move {
        for i in 50..100 {
            tx_b.send(i).await;
        }
    });

    loop {
        select! {
            Some(i) = rx_a.recv() => {
                println!("{}", i);
            }

            Some(i) = rx_b.recv() => {
                println!("{}", i);
            }
        }
    }
}
```
Output is similar to the previous example and contains numbers 1 to 49 and 50 to 99 in random order, depending on when they were sent onto the channel, depicting the concurrent nature of processes running in the tokio tasks:
```bash
$ ./run
1
2
3
50
51
52
...
```

There's a lot more complex, but fun stuff that happens here, and a lot of this is abstracted away to make the experience a breeze for beginners, but it's still well worth the time to delve into the guts of this exciting coding paradigm. I for one, will be continuing to learn a lot, that's for sure, until the next one, farewell friends :D