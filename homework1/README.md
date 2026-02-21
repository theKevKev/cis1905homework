# Homework 1: Directed Graph

For this homework, you will implement a basic directed graph structure.

## Problem Description

Rust's ownership model makes it notoriously difficult (if not impossible) to
implement some "traditional" data structures such as **linked lists** and
**graphs**. The problem lies with ownership.

In Rust every value has an owner. But who owns who in a graph? Consider the
following example:

```
|--------|          |--------|
| Node A | -------> | Node B | 
|--------|          |--------|
    ^                    |
    |                    |
    |                    v
|--------|          |--------|
| Node D | -------> | Node C | 
|--------|          |--------|
```

If we try to have nodes **own** their children, we end up with the following
ownership relationships:

```
        owns           owns           owns           owns
Node A ------> Node B ------> Node C ------> Node D ------> Node A  (!!)
```

Rust requires every value has _one_ owner, so we will never be able to construct
a graph like this.

> Note: we can still construct trees! Just not _arbitrary_ graphs.

### References to the rescue?

Your next thought may be to use references. But it turns out this doesn't work
either...

If we're going to use references, we need to choose between **immutable** or
**mutable** references.

- If we use **immutable** references, we won't be able to mutate our graph
  nodes.
- If we use **mutable** references, we have the same problem we did with
  ownership---mutable references are guaranteed to be **exclusive**, meaning we
  can never form cyclic graphs.

Furthermore, embedding references within `struct`s requires a concept we haven't
yet discussed (lifetimes). So in summary, I strongly recommend **avoiding**
references within `struct`s altogether.

### So how do I do it?

The challenge here is to come up with a creative solution to model graphs in
Rust. However, we will give you a few hints:

- Avoid thinking in terms of objects and references; instead, think about
  **where** the data lives.
- Think about the different responsibilities of the `Graph` and `NodeId` types
  (and possibly your own third type!!).
- You're going to need some kind of collection---see the
  [cheatsheet](#collections-cheatsheet) (below) for some usage examples.
- Our solution didn't involve any references within structs (just ownership).

## Starter Code

You will be provided with two files. Most of your code will go in `graph.rs`,
which is the Rust "module" that will contain your implementation. A test suite
will be provided in `tests.rs`, and you will be free to modify `main.rs` to
write your own quick tests as well.

- To run the test suite, simply run `cargo test`. A test report will be
  generated showing which ones fail.
- To run the `main()` function (in case you put code there), simply use
  `cargo run`.

> Note: although they will not be graded, feel free to add your own tests! They
> are very easy to create in Rust: adding the `#[test]` attribute to a function.
> The autograder will use its own version of the tests based on the original
> ones provided. For quick experimentation, also consider using the `main()`
> function.

Unlike some of your other classes, we aren't asking you to implement any fancy
graph algorithms here. Instead, the emphasis is on getting familiar with Rust's
memory management model through a fun and thought-provoking exercise.

We're providing a basic API that you will need to implement, documented below:

```rust
#[derive(Debug)]
pub struct Graph {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {}

// ^^^ Feel free to add your own structs as well ^^^

// This is called an "impl block", which implements functions associated with the `Graph` type.
// Within here you will see the `self` keyword, which refers to an implicit first argument.
// - e.g. calling `my_graph.get(id)` is equivalent to `Graph::get(&my_graph, id)`
#[allow(unused_variables)]
impl Graph {
    /// Constructs a new  graph. To call this function use `Graph::new()`.
    pub fn new() -> Graph {
        unimplemented!("new");
    }

    /// Adds a node to the graph with the given value, returning a unique id
    pub fn add(self: &mut Graph, value: String) -> NodeId {
        unimplemented!("add");
    }

    /// Returns true if the graph contains the given id
    pub fn contains(self: &Graph, id: NodeId) -> bool {
        unimplemented!("contains");
    }

    /// Removes a node from the graph, as well as any edges associated with it
    pub fn remove(self: &mut Graph, id: NodeId) {
        unimplemented!("remove");
    }

    /// Immutably borrows the value stored in a node
    pub fn value(self: &Graph, id: NodeId) -> &String {
        unimplemented!("value");
    }

    /// Mutably borrows the value stored in a node
    pub fn value_mut(self: &mut Graph, id: NodeId) -> &mut String {
        unimplemented!("value_mut");
    }

    /// Adds a directed edge from the `from` node to the `to` node (if it doesn't exist)
    pub fn connect(self: &mut Graph, from: NodeId, to: NodeId) {
        unimplemented!("connect");
    }

    /// Removes a directed edge from the `from` node to the `to` node (if it exists)
    pub fn disconnect(self: &mut Graph, from: NodeId, to: NodeId) {
        unimplemented!("disconnect");
    }

    /// Returns a list of (valid) node ids present in the graph
    pub fn ids(self: &Graph) -> Vec<NodeId> {
        unimplemented!("ids");
    }

    /// Returns the list of (out) neighbors of a node
    pub fn neighbors(self: &Graph, id: NodeId) -> Vec<NodeId> {
        unimplemented!("neighbors");
    }
}
```

> Note: make sure to leave in the `#[derive]` attributes, as they will make
> using your graph much more ergonomic (for example, by allowing `NodeId` to be
> implicitly copied and compared with `==`, etc.).

> Note: `//` are normal comments in Rust whereas `///` are "documentation"
> comments. Try out building documentation with `cargo doc --open`!

## Important Considerations

### Reusing Storage

You should make sure not to "leak" string memory. That is, when a node is
**removed**, the `String` associated with it should be freed or cleared via
`.clear()`. One of the tests will be a stress-test for this exact scenario.

This creates some interesting design challenges, including...

#### The ABA Problem (important!)

One problem you might encounter with your implementation is the ABA problem.
Suppose you create some node `A`, remove it, then create a new node `B`. If `B`
re-uses the same storage as `A`, you might unintentionally read `B`'s data when
accessing the graph via `A`'s (outdated) `NodeId`.

The required behavior is for your code to **panic** (crash) if an outdated
`NodeId` is used anywhere (i.e., one that's been previously removed from the
graph). Think about how you can detect this situation...

```rust
// make a new graph
let mut graph = Graph::new();

// add and remove a node
let a = graph.add("Node A".to_owned());
graph.remove(a);

// add a new node (and suppose it happens to re-use A's storage)
let b = graph.add("Node B".to_owned());

// this should `panic!()` -- if you haven't solved the ABA problem, it prints "Node B"
println!("{}", graph.value(a));
```

> Note: the easiest way to panic your program is by calling the `panic!()`
> macro. You need to find a way to detect the ABA problem at runtime and crash
> appropriately.

### A few subtleties

Do not change the public API of `Graph`! The autograder relies on this!

> You may choose to _add_ functions, which is okay. Just don't modify/delete the
> existing ones.

You don't need to handle the case where a `NodeId` from one graph is used for
another graph. In that case, your code's behavior is left unspecified.

If you suspect the stress test is timing out but your code is correct, consider
increasing `STRESS_TEST_TIMEOUT` in `tests.rs` (the default value of 3 seconds
_should_ be long enough, but I'm not sure).

> Note: although Rust cleans up heap memory for you, it is still possible to
> "leak" memory by holding onto objects for too long, as memory is managed by
> the ownership system. This is considered "safe" behavior as it can't cause a
> memory safety _bug_, but it can be wasteful.

## Debugging Tips

Print debugging in Rust is actually quite powerful due to a trait called the
`Debug` trait. We haven't really discussed traits yet (other than briefly
mentioning the `Copy` trait), but using `Debug` is actually very easy:

First, add `#[derive(Debug)]` to any structs you want to be able to debug-print.
For example:

```rust
#[derive(Debug)]
struct Graph { ... }
```

Then, use one of the following methods for print debugging:

```rust
// take note of the `:?` following `my_graph` (this indicates a "debug" print)
println!("{my_graph:?}");

// simpler, also prints the file name and line number for you
dbg!(&my_graph);
```

This lets you easily print out the contents of any data structure (including
`Vec`, `HashMap`, etc., which have their own `Debug` implementations).

## Submission and Grading

Submission will be on **Gradescope**, due on Monday, February 23rd at 11:59 PM.
Grading will be as follows:

- Autograder---90%
- Clippy---5%
- Formatting---5%

> Note: solutions that do not compile will receive no credit

[Clippy](https://doc.rust-lang.org/stable/clippy/) is a linting tool that is
built into cargo. You can run it with `cargo clippy`, and it will report
warnings for you. Make sure your code is not triggering any lints to get the
full points!

Formatting is easy in Rust with
[rustfmt](https://rust-lang.github.io/rustfmt/?version=v1.8.0&search=import).
You can run it with `cargo fmt`, which will format your code according to a
standard specification.

## Usage of AI

Usage of AI on this assignment **is not prohibited** (see the
[syllabus](https://www.cis.upenn.edu/~cis1905/2026spring/syllabus.html) for
general information regarding AI).

**However**, I do have some general tips with regards to AI usage and getting
the most out of the class:

- I actually _encourage_ you to use AI tools for help with compiler errors,
  syntax, and using standard library types. In my experience AI has been
  extremely useful in explaining errors and general context about how Rust
  works, especially for beginners.
- However, I would _strongly recommend_ against using AI to just write the
  entire assignment, as the main goal here is to get comfortable with Rust's
  different "mode" of thinking about programs and data structures.
- If you use AI autocomplete tools such as GitHub Copilot, I recommend disabling
  them for at least part of the assignment. Instead, I strongly recommend
  getting your editor set up with
  [rust-analyzer](https://rust-analyzer.github.io/), which is extremely useful
  for discovering methods on standard library types and identifying compilation
  errors.

# Collections Cheatsheet

Also feel free to reference the
[docs](https://doc.rust-lang.org/std/collections/).

> Note: these collections use a concept we haven't yet discussed (generics).
> Fortunately, Rust is able to infer generic types most of the time, but we'll
> also include examples with type annotations.

All these collections **own** their elements. This will be important to consider
when building your graph.

## `Vec<T>`

**In short**: `Vec` is a growable list of type `T` backed by heap memory. It's
analogous to C++'s `std::vector` and Java's `ArrayList`.

You don't need to import `Vec` it as it's included in the Rust "prelude" (a
collection of standard library types that are deemed "essential" enough to be
imported by default).

There's two main ways to create a `Vec`:

```rust
let mut my_vec: Vec<String> = Vec::new();          // creates an empty vector
let mut my_vec: Vec<String> = vec!['x', 'y', 'z']; // populates with some initial data
```

To add elements to a vector, we have the following methods:

```rust
my_vec.push(42);       // pushes an element to the end/back of the vector
my_vec.insert('x', 3); // inserts an element at a specific index
```

To remove elements, we have the following methods:

```rust
let last_elt: Option<String> = my_vec.pop();  // tries to pop an element from the end
let last_elt: String = my_vec.pop().unwrap(); // pops an element from the end (or panics)
let elt = vec.remove(6);                      // removes an element by index (panics if OOB)
```

> Note: we have a section discussing the `Option<T>` type as well, which is
> common in the standard library

To access elements, we have the following methods:

```rust
// the following operations panic (crash) if index is out of bounds
let by_value = vec[4];    // takes an element by value (requires that T is Copy)
let by_ref = &vec[5];     // immutably borrows an element
let by_mut = &mut vec[5]; // mutably borrows an element

// the following methods return an optional value
let by_ref_opt = vec.get(5);     // immutably borrows an element
let by_ref_opt = vec.get_mut(5); // mutably borrows an element
```

Also useful:

```rust
let length = my_vec.len(); // returns the number of elements
my_vec.clear();            // removes all elements, resets length to 0

if my_vec.contains('l') { ... } // true if the vector contains the given value
```

To iterate over a vector, there are a few options:

```rust
// iterate by value (consumes the vector)
for elt in my_vec { ... }

// iterate by reference
for elt in &my_vec { ... }

// iterate by mutable reference
for elt in &mut my_vec { ... }

// iterate directly over indexes
for idx in 0..my_vec.len() { ... }

// iterate over (index, value) pairs (immutably/mutably, respectively)
for (idx, elt) in my_vec.iter().enumerate() { ... }
for (idx, elt) in my_vec.iter_mut().enumerate() { ... }
```

## `HashMap<K, V>`

**In short**: `HashMap` is a mapping from keys of type `K` to values of type
`V`. It's analogous to C++'s `std::unordered_map` and Java's `HashMap`.

In order to use `HashMap`, you'll need to import it. Just add the following line
to the top of your `graph.rs`:

```rust
use std::collections::HashMap;
```

The main way to create a `HashMap` is via `HashMap::new()`.

```rust
let mut map: HashMap<String, i32> = HashMap::new();
```

To insert values:

```rust
// inserts a (key, value) pair into the map
map.insert(String::new("Hello, world!"), 42);
```

To get (borrow) a value:

```rust
let by_ref: Option<&i32> = map.get(&key);     // borrows a value immutably
let by_mut: Option<&i32> = map.get_mut(&key); // borrows a value immutably
```

To remove (take) a value:

```rust
let by_value: Option<i32> = map.remove(&key); // tries to remove a value, returns an option
```

To get the length (number of entries), use `map.len()`. `HashMap` also provides
useful iteration capabilities:

```rust
for (k, v) in my_map { ... }      // iterate by value (consumes the map)
for (k, v) in &my_map { ... }     // iterate by reference
for (k, v) in &mut my_map { ... } // iterate by mutable reference

for k in my_map.keys() { ... }       // iterate by reference over keys
for v in my_map.values() { ... }     // iterate by reference over keys
for v in my_map.values_mut() { ... } // iterate by mutable reference over keys
```

> Note: these iterators are in "arbitrary" order, so you shouldn't rely on them
> being in any particular order

## `Option<T>`

This is not a collection, but rather a type which is used to indicate an
"optional" value. It's analogous to C++'s `std::optional` and Java objects
(which are implicitly nullable).

This type is included in the Rust prelude, as well as its variants `Some` and
`None`.

Here's some usage examples:

```rust
// create an optional value with no value
let mut my_opt: Option<i32> = None;

// put a value in the option
my_opt = Some(10);

// match against the option, handling each case
match my_opt {
    Some(value) => println!("the value is {value}"),
    None => println!("there is no value"),
}

// "unwrap" the option, getting the inner value (and panicking if it's not present)
let inner = my_opt.unwrap();
let inner = my_opt.expect("error message if it fails");

// safely unwrap by providing a "default" value
let inner = my_opt.unwrap_or(10);

// check if the option has a value (so we can safely unwrap)
if my_opt.is_some() {
    println!("{}", my_opt.unwrap());
}

// check if the option is empty
if my_opt.is_none() {
    return;
}
```

This style of code should be sufficient for the purposes of the homework. We
will see much more about `Option` (and `enum`s in general) going forward in the
class!
