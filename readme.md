# RostFix: A Simple Stack Language
RostFix is a stack-based language heavily inspired by a language PostFix in the book "Design Concepts in Programming Languages" (https://mitpress.mit.edu/9780262201759/design-concepts-in-programming-languages/).

Overall, RostFix works like a stack machine. 

It remains a work in progress.

## Grammar
A code is a sequence that starts with the token `rostfix`, the number of parameters and RostFix commands.
If we have a program `rostfix 0 9 3 4 add`, `0` means the number of parameters passed, and `9 3 4 add` is the body of commands.

During the execution, RostFix adds `9`, `3` and `4` to a stack and with `add`, it pops last two values (`3` and `4`) from the stack and calculates `3 + 4`.

### Example

`sample.rf`
```
rostfix 1 3 add
```

In console
```
cargo run sample.rf 1
=> 4
```
