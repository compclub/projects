# Chapter 2: Combinators

I don't find toy programming examples like those in the book so far very convincing. So I'm going to
try applying the approaches in the book in settings that are bigger or harder or more interesting.

First up: combinators from chapter 2. I apply the combinator approach to an actual Regex engine in
Rust.  It is unicode aware, quite fast (~7ns/char parsed in a microbenchmark), and _open to
extension_. Specifically, you can import the library, implement your own regex combinator, and mix
and match it with the existing combinators.

My conclusion is that you could actually make a user extensible industrial strength regex library
out of combinators. And use it in production and everything would be fine.

-- Justin Pombrio

## What are Combinators all about?

The essense of combinators is that they all _share the same interface_. This makes them easy to
extend: to write your own combinator, all you have to do is satisfy that interface.

For example, the interface of the regex combinators in the book is a string which is a valid POSIX
regex. (This should make it clear that an interface is not merely about types: most type systems
would merely say "string", but the books combinators will break if you mix in a combinator that
makes non-POSIX-regex strings.)

As another example, a simple set of parser combinators might share the interface (using Haskellish
syntax):

    type Parser a = String -> Maybe (a, String)

That is: a parser of type `a` takes in a string, and is either successful or not. If it is
successful, it produces a pair of an output `a` and the _unparsed suffix_ of the string. This
unparsed suffix is what allows parser combinators to be chained together. If it is not successful,
it produces nothing.

I would make a distinction between two approaches to combinators, that's not mentioned in the book:

- Combinators can be _open to extension_, meaning that a user of the combinators can write their own
  combinator (that's not merely a composition of the existing ones). For example, Rust's [nom
  parser combinator library](https://docs.rs/nom/6.1.2/nom/index.html) is open: a nom combinator is
  roughly a function with the signature:

          fn parse(&str) -> Result<(&str, &str), Err<Error<&str>>>

  where `Err` and `Error` are types defined in the library.
- Combinators can also be _closed to extension_, meaning that you can _only_ use the provided
  combinators, and cannot construct your own. For example, Haskell's [Parsec parser combinator
  library](http://hackage.haskell.org/package/parsec-3.1.14.0/docs/Text-Parsec.html) is closed to
  extension. You cannot as far as I can tell add your own custom combinators.

"Open to extension" sounds better, and I think it's more in line with the book's aims, but I think
that "closed to extension" can be just fine if your combinator set extensively covers everything
anyone could want to do that's compatible with the shared interface. No one ever says "I'm so sad
that the `Parsec` type is closed, there's a parser combinator I want that's compatibe with the
`runParserT` type signature but cannot be constructed out of the existing combinators."

Since it's more in line with the book, I tried writing Regex combinators that are _open to
extension_, in Rust.

## My Regex Combinators

My Regex combinators have this interface (`trait` in Rust means "interface"):

    pub trait Regex: Clone {
        fn initialize(&mut self);
        fn start(&mut self);
        fn advance(&mut self, ch: char);
        fn accepts(&self) -> bool;
        // This is just an optimization:
        fn is_dead(&self) -> bool;
    }

The details are in the docs in `src/lib.rs`, but that should give you a sense of it.  This interface
pushes you towards an NFA-like implementation, which allows `O(NM)` parsing time, where `N` is the
size of the input and `M` is the size of the regex.  Determining this interface was by far the
hardest part of this exercise: the actual implementation followed naturally.

My library is _open to extension_. Users are free to implement the `Regex` trait themselves, and the
existing combinators will happily combine their new combinator.

For performance testing, I implemented the regex `^(0|[1-9][0-9]*)(\\.[0-9]*)?$`, tested it on a
couple length 50 strings, and compared to Rust's `regex` crate, which is probably as fast as you can
get. The results on my laptop are:

                *-------------*------------------*
                | regex crate | combinator_regex |
    *-----------*-------------*------------------*
    | time      | 227 ns      | 666 ns           |
    *-----------*-------------*------------------*
    | time/char | 2.7 ns      | 6.6 ns           |
    *-----------*-------------*------------------*
    | memory    | 4000 bytes  | 64 bytes         |
    *-----------*-------------*------------------*

## To Run

Install Rust:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Run the tests:

    cargo test

or run the benchmarks:

    cargo bench

or build and open the docs:

    cargo doc --open
