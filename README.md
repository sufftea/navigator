# Motivation

Typing out the entire path when `cd`ing, even with tab completion, is 
inconvenient. It would make a lot of sence to use a fuzzy picker for that. 
There is a tool that does something similar called zoxide, but it only lets 
you search for paths that you've already opened. I found that inconvenient, so 
I had to build my own.

My first attempt was this: `cd $(find ~ | fzf)`. The problem was that `find` 
performs a depth-first search, which means that if, say, you want to navigate 
to a directory of depth 2, you have to wait until `find` scans through a 
million obscure, deeply nested directories you don't care about. This was 
taking too much time.

Then I wrote a shell script that runs `find` in a breadth-first manner. The 
problem here was that after selecting the directory with `fzf`, `find` kept 
running for a while, creating another delay. There probably is a solution to 
that, but I suck at bash, so I switched to Rust, which I suck at a little less.

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



