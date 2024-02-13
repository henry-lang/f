# Conjuctions
\ignore a -> 0
\both a b -> + ignore a ignore b

# Recursive Sequences
\fac n -> if = n 0 1 * n fac - n 1
\fib n -> if = n 0 1 if = n 1 1 + fib - n 1 fib - n 2
\main -> both both print + fac 20 fib 3 print 3 print 4
