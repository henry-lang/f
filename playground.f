# Conjuctions
\do_both a b -> none

\print_to_inner max n -> 
    if = n max
        none
        do_both print + n 1 print_to_inner max + n 1
\print_to max -> print_to_inner max 0
\main -> print_to 100
