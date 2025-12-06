# Motivation

Typing out the entire path when `cd`ing, even with tab completion, is 
inconvenient. It would make a lot of sense to use a fuzzy picker for that. 
There is a tool that does something similar called 
[zoxide](https://github.com/ajeetdsouza/zoxide), but it only lets you search 
for paths that you've already opened. I found that inconvenient, so 
I had to build my own.

My first attempt was using `cd $(find ~ | fzf)`. The problem was that `find` 
performs a depth-first search, which means that if, say, I wanted to navigate 
to a directory of depth 2, I had to wait until `find` scans through a 
million obscure, deeply nested directories I don't care about.

To fix that, I tried to set up a [shell script](https://unix.stackexchange.com/questions/279895/how-can-i-do-a-breadth-first-search-using-find) 
that runs `find` in a breadth-first manner, but now, after selecting the 
directory with `fzf`, `find` kept running for a while, creating another delay. 
There probably is a solution to that within bash, but I suck at bash, so I 
switched to Rust, which I suck at a little less.

This tool does the same thing as that script: runs `find` in a loop and pipes 
its output to `fzf`.

# How to use

- Have `find` and `fzf` installed.
- Build the project.
- Make it runnable globally.
- Add this to `.bash_aliases`:
  ```
    # Search under home dir
    alias ,='cd "$(navigator ~)"'
    # Search under current dir
    alias ,c='cd "$(navigator .)"'
  ```



