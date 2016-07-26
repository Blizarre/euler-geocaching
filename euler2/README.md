#euler 2

I wanted to solve this problem in bash as a challenge, but it wasn't such a great idea (who knew?).

Bash arithmetic is limited to 32/64 bits integers, which is not enough to solve this problem. 
I had to use the command-line tool `bc` to do the computations, but it makes the code very slow.

As the numbers generated are very long, I wrote a small parser script to extract the central digits for the geocaching puzzle.
