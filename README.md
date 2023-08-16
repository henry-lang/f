# f
Some sort of goofy functional programming language. Here's how you can define factorial(n):

```
fac n -> if = n 0 1 * n fac - n 1
```

Currently, side effects are a bit weird but you can write a runnable program to execute outside of the shell by defining a main function such as this:

```
main -> print "Hello, world!"
```

The print function is impure and returns a `Nothing` type, and there may be an impure keyword added in the future to help document programs.
