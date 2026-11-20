# 2. Palindrome

## Problem

Write:

```rust
fn is_palindrome(input: &str) -> bool
```

Decide whether the input is a palindrome.

Do **not** start from a ready-made solution that simply reverses the string.

Examples:

```text
"abba"   -> true
"abcba"  -> true
"hello"  -> false
```

Practice: `&str`, UTF-8, `chars()`, iterators, borrowing.

## Acceptance criteria

- [ ] ASCII palindromes and non-palindromes are classified correctly.
- [ ] UTF-8 input is handled via Unicode scalar values (`char`), not raw byte indexes.
- [ ] The function borrows the input; it does not take ownership of a `String` unless needed.
- [ ] Tests exist, and the code compiles warning-free.
- [ ] You can explain the difference between a byte index, a `char`, and a Unicode scalar value.
