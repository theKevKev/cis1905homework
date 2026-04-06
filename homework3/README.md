# Homework 3: Multi-Producer, Multi-Consumer Channel

For this homework, you will implement an unbounded multi-producer,
multi-consumer (mpmc) channel in Rust. Then, you will implement a static HTTP
web server using your channel implementation.

The implementation is mostly open-ended, and we're only providing you with basic
starter code consisting of some types and function signatures (such that your
code will be compatible with the test suite).

## Requirements Overview (MPMC Channel)

You are implementing an **unbounded** multi-producer, multi-consumer channel.
This means we can have multiple senders and multiple receivers that reference
the same underlying communication channel.

Since the channel is unbounded, sending is always instantaneous. That is, we
never need to block to wait for the channel to have space available. But
_receiving_ can block in general, so we have two functions:

- `try_recv`, which tries to receive a value immediately without blocking,
  otherwise returning `Empty`
- `recv`, which blocks (waits) until a value is available on the channel, if
  necessary

You are tasked with implementing the following types and functions in `mpmc.rs`:

```rust
/// Opens up a communication channel over type `T`, returning a linked sender and a receiver.
pub fn channel<T>() -> (Sender<T>, Receiver<T>)

/// The "sender" end of a channel (fields must not be marked `pub`).
pub struct Sender<T> { ... }

/// The "receiver" end of a channel (fields must not be marked `pub`).
pub struct Receiver<T> { ... }
```

In addition, you will need to implement three channel methods (`send`,
`try_recv`, and `recv`):

```rust
impl<T> Sender<T> {
    /// Sends a value over the channel, returning immediately.
    /// Returns `Err(T)` if the receiver end is disconnected.
    pub fn send(&self, val: T) -> Result<(), T> { ... }
}

impl<T> Receiver<T> {
    /// Tries to receive a value over the channel, returning immediately.
    /// Returns `Err(TryRecvError::Empty)` if the channel has no data right now.
    /// Returns `Err(TryRecvError::Disconnected)` if the sender end is disconnected.
    pub fn try_recv(&self) -> Result<T, TryRecvError> { ... }

    /// Tries to receive a value over the channel by **blocking**.
    /// Returns `None` if the sender end is disconnected.
    pub fn recv(&self) -> Option<T> { ... }
}
```

You are provided the `TryRecvError` type, defined as such:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TryRecvError {
    Empty,
    Disconnected,
}
```

### Other Constraints

To maintain the spirit of the assignment, you are expressly **forbidden** from
using the following:

- Using `std::mpsc` or any other channel primitive to implement your own channel
- Using `loop` (or similar) to
  [busy-wait](https://en.wikipedia.org/wiki/Busy_waiting) for the channel to
  have data available, which wastes cycles
- Using any third-party crates

The use of other standard library types (`Arc`, `Mutex`, `RwLock`, `Atomic*`,
`Condvar`, etc.) is encouraged (you'll likely need at least some of them to
implement your channel!).

Furthermore, you must not leak resources. That is, if the senders and receivers
are dropped, you _must_ clean up the underlying channel.

> This will probably be the case as long as you don't use cyclically-referenced
> `Arc` or `Box::leak`.

## Requirements Overview (Static Web Server)

The web server implementation is quite open-ended, but you'll be using the
following public interface:

```rust
/// A running web server (fields must not be marked `pub`)
pub struct HttpServer { ... }

impl HttpServer {
    /// Opens and starts up a web server on the given port (bound to `0.0.0.0`).
    /// Starts up `num_workers` worker threads to handle requests.
    #[must_use]
    pub fn new(port: u16, num_workers: u32) -> Self { ... }
}
```

Your web server must have the following semantics:

1. The web server is automatically started after calling `HttpServer::new`
2. The web server (and associated threads) are eventually stopped after
   `HttpServer` is dropped
3. Concurrent request handling should be supported

To test concurrent request handling, we suggest adding a call to
`std::thread::sleep` within your request handler, then testing within the
browser to make sure those sleep timers can run independently.

Your server will be responsible for serving static files in the project's
current working directory. For example, the user should be able to request
`/hello.txt`, and your server should return the contents of that file as
plaintext as part of the response body.

If the file exists, return a `200 OK` response with the file contents, and the
`Content-Type` header set to `text/plain`. Otherwise, you should return
`404 Not Found` with an appropriate error message.

You won't need to implement any advanced error handling, and may assume the
client will send well-formed HTTP requests.

### HTTP/1.1 Protocol

We don't expect you to be an expert on the HTTP protocol, so here's a
(simplified) summary of what to do:

Firstly, HTTP runs on top of TCP, so you'll need to listen for incoming
connections. The entire protocol is **plaintext**, which makes parsing simple.

When a client connects, they'll send something that looks roughly like this:

```
GET /contact HTTP/1.1
Host: example.com
User-Agent: curl/8.6.0
Accept: */*
```

For the protocol to work, you'll need to read in the first line of the request
(e.g. `GET /contact HTTP/1.1`) **and** all of the headers (e.g.
`Host: example.com`). In Rust, you can create a `BufReader` over your
`TcpStream` and read in all the lines until you encounter (and consume) an empty
line.

> For the purposes of the assignment, you can **ignore** the headers entirely.

To parse the request, you can just extract the second word from the first line
(e.g. `/contact`). That's the file you'll need to serve to the client.

The response format will look roughly like this:

```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 12

Hello, world!
```

> The `Content-Type` header can just be set to `text/plain` in your response.
> `Content-Length` should be the length (in bytes) of the response body, which
> is written after two CRLF newlines. This tells the client how much data to
> read after finishing reading the headers (the end of which is indicated by
> double CRLF).

**Important**: you can't use normal newlines in HTTP, instead you'll need to end
each line with `\r\n` (CRLF). We suggest manually writing these characters out
in Rust instead of using the `writeln!` macro or similar.

If the file doesn't exist, you should return a 404 response, for example:

```
HTTP/1.1 404 Not Found
Content-Type: text/html
Content-Length: 14

File not found
```

You may want to (but aren't required to) craft a more helpful error message,
such as including the file path in the response for debugging purposes. We'll
only be checking the status code (not the response body or content type).

# Suggested Approach

The implementation is entirely up to you, but we're also providing detailed
guidance:

> Note: our test cases are organized into "phases" which correspond to those
> below

## Phase 1: Basic Sending and Receiving

You'll want to start by establishing some kind of shared state for a channel. We
recommend creating a struct called `ChannelInner<T>`, which holds all the shared
internal state for a channel. Then, your `Sender<T>` and `Receiver<T>` types can
each have a single field called `inner` of type `Arc<ChannelInner<T>>`. This
approach has a number of advantages:

1. You only need one instance of `Arc` (instead of one for each shared field)
2. It makes it easy to extend/change this shared state in the future

So in memory, what we have is something that roughly looks like this:

```
┌────────────┐                                  ┌────────────┐
│            │                                  │            │
│   Sender   ├───┐                          ┌───┤  Receiver  │
│            │   │                          │   │            │
└────────────┘   │                          │   └────────────┘
┌────────────┐   │    ┌────────────────┐    │   ┌────────────┐
│            │   │    │                │    │   │            │
│   Sender   ├───┼───►│  ChannelInner  │◄───┼───┤  Receiver  │
│            │   │    │                │    │   │            │
└────────────┘   │    └────────────────┘    │   └────────────┘
┌────────────┐   │                          │   ┌────────────┐
│            │   │                          │   │            │
│   Sender   ├───┘                          └───┤  Receiver  │
│            │                                  │            │
└────────────┘                                  └────────────┘
```

Since `Arc` only permits shared references, you'll need to use interior
mutability within `ChannelInner`. This includes the shared buffer you'll use to
store channel items. We suggest using `Mutex<VecDeque<T>>`, but other data
structures will also work.

With this setup in place, you should be able to trivially implement `send` and
`try_recv` by locking the mutex, and calling `push_back`/`pop_front` on the
shared buffer.

Once you're done implementing this phase, run `cargo test` and make sure you're
passing all of `phase1::*`.

## Phase 2: Channel Cloning and Disconnection

The next step is to implement the logic handling **cloning** and **dropping** of
channels. This isn't quite as simple as just slapping `#[derive(Clone)]` on the
senders and receivers, since we need to handle the case where a channel becomes
"disconnected". Specifically, we define "disconnected" as following:

- The **sender end** becomes disconnected when all senders have been dropped
  **and** the channel is empty
  - Future calls to `Receiver::try_recv` must return
    `Err(TryRecvError::Disconnected)`
- The **receiver end** becomes disconnected when all receivers have been dropped
  - Future calls to `Sender::send` must return `Err(T)` with the original value
    (instead of dropping it)

We have this behavior to prevent one end of the channel from sending/receiving
"into the void". In many cases, we expect the user to just `.unwrap()` the
result.

To facilitate this behavior, you'll need four implementations:

- `impl<T> Clone for Sender<T>`
- `impl<T> Drop for Sender<T>`
- `impl<T> Clone for Receiver<T>`
- `impl<T> Drop for Receiver<T>`

**Hint**: you'll likely need to introduce some additional state to your
`ChannelInner` struct, such as a counter for the number of senders or receivers.
Two possibilities are to use a `Mutex` or `Atomic*` types.

> Note: if you choose to use atomics, you'll need to provide an `Ordering` for
> the atomic operations. This tells the hardware how it's allowed to reorder
> operations across different threads. If you don't know what this means, just
> use `Ordering::SeqCst` and everything should work.

Once you're done implementing this phase, run `cargo test` and make sure you're
passing all of `phase2::*`.

## Phase 3: Blocking Receive

The final step for implementing the channel is implementing the blocking
`Receiver::recv` method. This is a bit tricky, since we're not allowed to
busy-wait for the channel to have data available. There's several ways to do
this, but we suggest using the standard library `Condvar`.

> Note: `Condvar` implements a
> [condition variable](https://en.wikipedia.org/wiki/Monitor_(synchronization)),
> which gives you the ability to "wait" on an event and "notify" listeners when
> that event has happened. You're not expected to know this in depth, and we'll
> walk you through how to use it below. Feel free to read more about it
> [here](https://doc.rust-lang.org/std/sync/struct.Condvar.html).

To start, add a field of type `Condvar` to your `ChannelInner` struct. Then, in
your `send` implementation, check if the shared buffer is empty. If it was empty
**before** calling `push_back`, then you should call `push_back`, then call
`.notify_all()` on the condition variable (after pushing). Otherwise, you should
just `push_back` as normal. This will notify waiting threads that data has just
become available.

To implement `recv`, you can start by calling `try_recv()` and matching on the
result. If there's a result immediately, or the channel is disconnected, then
you can return immediately. Otherwise, you'll need to _wait_ on the condition
variable.

Implement the `recv` function, which blocks until data is available. We suggest
using the `.wait_while()`, which takes a mutex and a _closure_ (function
callback). What this does is wait until the variable is notified, then check the
condition, continuing to wait if it's `false` or proceeding if it's `true`.

The mutex argument (first argument) should be the result of calling
`.lock().unwrap()` on the mutex protecting your shared buffer. In your
condition, you should think about what cases it makes sense to stop waiting
(i.e., return something from `recv`). Importantly, the `.wait_while()` function
**returns** a mutex lock guard which you can use to access the shared buffer.
For example, if your channel buffer is called `buf`:

```rust
let mut buf = self
    .inner
    .condvar
    .wait_while(self.inner.buf.lock().unwrap(), |buf| { <MY_CONDITION_HERE> })
// do something with `buf`
```

> Warning: don't use `loop` to busy-wait for the channel to be full. You will
> lose style points.

Once you're done implementing this phase, run `cargo test` and make sure you're
passing all of `phase3::*`.

## Phase 4: Static HTTP Server

At this point, you're done implementing the channel, so you can move on to
`http.rs`. The implementation of the web server is more open-ended, but we have
some hints:

> **Important**: you're expected to use _your own channel implementation_ for
> implementing the HTTP server (not standard library channels!)

1. You'll likely want to create two channels, serving two different purposes
   - A channel for sending a "stop request" to the listener thread, telling it
     to shut down
   - A channel for sending values of type `TcpStream`, representing client
     connections
2. You'll likely want to spawn your threads (via `thread::spawn`) inside the
   `new` function of `HttpServer`
   - One thread should be a "listener" thread which accepts new TCP connections
     by (1) creating a `TcpListener` that binds to `0.0.0.0:{port}`, (2) calls
     `listener.accept()` in a loop (until the stop request is received), and (3)
     sends the accepted connections on the channel
   - There should be $n$ additional threads which (1) call `.recv()` to get a
     `TcpStream`, and (2) handle those connections by reading/writing to the
     stream

To handle reading/writing to the stream, you can create two variables, one of
type `BufReader<&TcpStream>` and one of type `BufWriter<&TcpStream>`.

To read from the reader, you can call `.read_line(&mut line)`, which takes a
mutable reference to a string (where the result will be written), and returns
the number of bytes read.

> Hint: to parse the request, you'll want to save the first line into a variable
> and call `.trim()` to remove trailing whitespace (CRLF). Then you can continue
> reading lines until the number of bytes read is `0` or
> `line.trim().is_empty()` returns true.

From the first line of the request, you can extract the request's URL. To read
the file relative to this location, you can do the following quick solution:

1. Remove the leading `/` by writing `let url = &url[1..]` (where `url` is the
   path part of the request)
2. Get a relative path by calling `Path::new(".").join(url)`
3. Make sure the path points to a file by calling `path.is_file()`, and handling
   the error case
4. Read in the file contents with `std::fs::read_to_string(path)`

To write the response, you can use the macro `write!()`, which accepts the
writer object (e.g. `BufWriter`), a format string, and an argument list. For
example:

```rust
write!(writer, "Content-Type: text/plain\r\n")?;
```

> Important: don't forget to add CRLF (`"\r\n"`) at the end of each line!

### Drop Logic

You'll need to implement `Drop` for your web server and clean up any associated
resources. For the purposes of this project, that mostly just means joining the
threads you've created, so you'll want to make sure they eventually stop running
(e.g. by sending a "stop request" over a channel).

### Error Handling

You don't need to implement any advanced error handling, but it will still be
helpful to catch and return errors. To make this easier, we suggest creating a
function (perhaps called `handle_conn`), which accepts a `TcpStream` by value
and returns a value of type `Result<(), MyErrorType>`. You can create an enum
for your error type, and one of the variants should be a thin wrapper over
`std::io::Error`. Then, you can implement `From<std::io::Error>` for your error
type. This will let you use the `?` operator to propagate errors within the
function. Essentially, what this does is:

1. When you write `expr?`, if `expr` is `Result::Ok(v)`, you just get the inner
   value `f`
1. When you write `expr?`, if `expr` is `Result::Err(e)`, the error value gets
   early-returned

Implementing `From<std::io::Error>` lets the returned error get implicitly
converted for you.

In your worker threads, you can match on the result of calling `hanlde_conn`,
and print out an error if it occurs. You also choose to implement
`std::fmt::Display` or `#[derive(Debug)]` on your error type to make this
easier.

### Testing

To test the implementation, you can just open your browser to `localhost:8000`
(or whatever port you choose), and add the following code to `main.rs`:

```rust
fn main() {
    // start up a server with 16 workers on port 8000
    let server = HttpServer::new(8000, 16);

    // delay dropping the server for a very long time
    std::thread::sleep(Duration::MAX);
    drop(server);
}
```

Specifically, you should make sure that:

1. The server is able to load static files and serve them to you as plaintext
2. Nonexistent files get served with a 404 error code

If it works consistently (i.e. no random "connection reset" errors, etc.), and
successfully serves responses, then you're good to go for this part.

You might also want to try something like
`curl http://localhost:8000/MY_FILE.txt`.

# Grading

Grading is broken down as follows:

- 50% autograder (same as provided test cases)
- 10% mpmc channel style points (e.g. not using busy wait, not using standard
  library channels)
- 25% web server functionality (manually graded by testing in the browser)
- 5% web server cleanup logic (making sure the threads get joined, you've
  implemented `Drop` correctly)
- 10% clippy and formatting

Solutions that don't compile will receive no points.

The homework is due Monday, April 6 at 11:59 PM via Gradescope.
