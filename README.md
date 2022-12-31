# Rustalgs

A program I made for trying to find better ZBLL algs.

It works using a BFS search for all provided triggers. The reason for using triggers is to have more control of which kind of algorithms are created, while also making the running time feasible.
It uses a 'meet in the middle' method, but can also be seen as pattern database or simlpy using a pruning table. 

To run this, you can clone the repo and do `cargo run --release` if you have rust installed. Otherwise, I have made an exe file for windows which should work assuming you place it in the same folder as `triggers.txt` and `case.txt`.
I have only tested on when windows when running it from cmd.

In `triggers.txt` you can specify all your triggers, one per line, that you what the program to try together with all other triggers. The file shouldn't have many more lines than what is currently in there until the running time becomes too long (the program tries roughly N^5 *2 combinations twice).

In `case.txt` you write an algorithm which solves the case you want other algs for.
