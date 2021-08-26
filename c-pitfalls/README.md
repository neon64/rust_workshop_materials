## Some buggy C code examples

[dangling-pointer.c](./dangling-pointer.c) should be clear to all who have done dynamic memory allocation etc...

[data-race.c](data-race.c) may make more sense if you've done some sort of multithreading before (e.g.: COMP30023)

### Compiling and running

Running

    $ make

should build both binaries.

Run the examples with

    $ ./dangling-pointer

And

    $ ./data-race