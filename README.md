
<!-- README.md is generated from README.Rmd. Please edit that file -->

# unextendr

<!-- badges: start -->
<!-- badges: end -->

## What’s this?

This is… nothing. I’m just re-inventing the wheel only to have slightly
better understanding about what [extendr](https://extendr.github.io/)
does.

## Random thoughts

### Error Handling

This framework uses [tagged
pointer](https://en.wikipedia.org/wiki/Tagged_pointer) to indicate the
error. This requires some wrappers, but the advantage is that we don’t
need to pass additional data and these bit operations should be cheap.

But, as (hopefully) [the stabilization of “C-unwind
ABI”](https://github.com/rust-lang/rust/issues/74990#issuecomment-1363473645)
is not very far, I’m not sure if this is worth considering to implement
in extendr.

See [extendr/extendr#278](https://github.com/extendr/extendr/issues/278)
for more discussion.

### Protection

I implemented the doubly-linked list method, which [cpp11
uses](https://cpp11.r-lib.org/articles/internals.html#protection). Now
I’m not sure if this fits extendr; probably its current implementation
aims for parallel processing, so it needs a hashmap to prevent
collisions. But, I think it’s not a good idea to use R’s C API
concurrently anyway, so this should be probably enough.

### Do we really want embedded usages?

Regarding the concurrency, I’m wondering if it would be simpler if
extendr give up supporting embedded usages. Yes, extendr is not only
about R packages. It also provides functionalities to embed R in Rust
like [this
one](https://github.com/yutannihilation/extendr-tide-api-server-example).
This is great, but, on the other hand, this means we have to care a lot
of things. But, how careful we try to be, concurrency is a tough job.

A lot of tests have failed because of various problems related to
concurrency. Every time we encountered such a failure, we place
`single_threaded()` here and there. But, could all of them really happen
inside an R package? If extendr gives up supporting embedded usages, can
our life be simpler a bit?

## Functions

``` r
library(unextendr)

to_upper("a")
#> [1] "A"
times_two_int(1L)
#> [1] 2
times_two_numeric(1.1)
#> [1] 2.2
```

## References

- <https://notchained.hatenablog.com/entry/2023/01/29/163013> (日本語)
