# f
Some sort of goofy functional programming language. Here's how you can define factorial(n):

```
fac n -> if = n 0 1 * n fac - n 1
```

# Syntax

The syntax is very minimal, as there are really only if expressions (to introduce lazyness so recursion doesn't halt forever):
`if cond tb fb`

Everything else is either just type literals such as numbers (`1`...) or strings (`Hello, world!`) or function application:

```
add a b -> + a b
```

This defines a function with two arguments `a` and `b`, and then the body starts after the arrow. Since + expects 2 arguments, there is no delimiter for the arguments.

# Side Effects

Currently, side effects are a bit weird but you can write a runnable program to execute outside of the shell by defining a main function such as this:

```
main -> print "Hello, world!"
```

The print function is impure and returns a `Nothing` type, and there may be an impure keyword added in the future to help document programs. I was thinking a tilde, so a main function would be `~main`.

# Type System

As of right now, the type system is very limited as there are only `Number`, `String`, `Boolean`, and `Nothing` types. `Number` is always floating point, and there do not exist any utility functions on `String`. `String` may be refactored into a `List Character` if I decide to add a `List Element` type, but then I would have to add generic type parameters and that would be a major change. `List` would be a linked list, probably implemented using the `im` crate for quick accesses and cloning.
