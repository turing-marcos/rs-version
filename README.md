# rs-version

A simple rust library for managing versions of a program compiled using Cargo.

The `from_str` function is based on the one by [nulldatamap](https://github.com/nulldatamap), who has a very similar crate but without de/serialization.

## Why?

Because I this is my third time doing it from scratch and I prefer having a separate crate that is well-tested and standalone.

This crate is useful for transmitting/receiving/saving data from a rust application: You save up the version together with the data and that lets you compare the compatibility of the programs.

### Why not use [nulldatamap](https://github.com/nulldatamap)'s "[version](https://github.com/nulldatamap/version)" crate instead of this?

His implementation does not have de/serialization nor the ability to compare versions. I also think that mine is slightly more convenient.

### Why not make a PR to [nulldatamap](https://github.com/nulldatamap)'s "[version](https://github.com/nulldatamap/version)" crate?

Because I think he was going for minimalism and simplicity, while mine leans more towards comfort and completeness.

Of course this is still very minimalistic, it's just a version struct after all, but nevertheless it's fundamentally different from his implementation (except for the `from_str` function as mentioned).

### So why would I use [nulldatamap](https://github.com/nulldatamap)'s "[version](https://github.com/nulldatamap/version)" crate?

As mentioned, his is leaner. His version will compile faster (Serialize/Deserialize traits add compilation time) and weigh less.

## How?

I implemented a custom serializer and deserializer, so that instead of being serialized as:
```JSON
{
   "major": 1,
   "minor": 0,
   "patch": 0
}
```
It will serialize as `1.0.0`. This makes much more sense for including it inside another struct and serializing it.
