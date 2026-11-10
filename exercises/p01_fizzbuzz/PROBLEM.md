# 1. FizzBuzz

## Problem

Write a function that walks the integers from 1 through `N`.

- divisible by 3 → `Fizz`
- divisible by 5 → `Buzz`
- divisible by both 3 and 5 → `FizzBuzz`
- otherwise → the number itself

Return the result as `Vec<String>`.

```rust
fn fizzbuzz(n: u32) -> Vec<String>
```

Practice: `for`, `if`, `match`, `String`, `Vec<T>`, ownership.

## Acceptance criteria

- [ ] `fizzbuzz` matches the mapping above for every `n`, including `0` and multiples of 15.
- [ ] The program has tests covering typical and edge cases.
- [ ] The code compiles warning-free.
- [ ] There is no unnecessary `clone()` and no extra heap allocation just to satisfy the borrow checker.
