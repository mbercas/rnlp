# rnlp
Natural Language Processing functions in Rust

## Objective
This is my first project in Rust, if fact it started as a simple function to test string slicing I copied learnt from one of the first chapters of the Rust tutorial and started evolving as I read more of the book and learnt some other features. 
I plan to keep learning Rust and adding more functionality here, in principle there is no real pla; Tokenization, WordVectors, text statistics, colocations, who knows.
At some stage I will compare the performance of my functions to those in the NLP library for Python. I hope to achieve better performance but who knows.
### Sub objectives
 - Use test driven development
 - Try CI in github
 - Refactor old code every time a new language feature is learnt
 - Generate documentation from code
### Caution
I make this repository public in case anyone is interested, but please be aware that for the time being I am at very early stages of Rust learning and things may be done in not exacly the best way.

## Learning links

The (https://doc.rust-lang.org/book/)[Rust Book].

### Dictionary of terms

 - *Monomorphization* is the process of turning /generic code/ that
 is using generics at compile time by filling in the concrete types
 that are used when compiled.
 - *Variant* one of the possible values an Enum can take

Comming from C/C++ world some Rust contructs have different
names in C++; some concepts are not the same.

| *Rust*        | *C++*          |
|---------------|----------------|
| Method        | Method         |
| Struct        | Struct / Class |
| Generic Types | Templates      |
|               |                |
