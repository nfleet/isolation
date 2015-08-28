# Isolation 

**Isolation** computes strongly connected components of a specified directed graph. 

```
Usage: isol [options] <SOURCE> <<TARGET>>

Options:
    -a, --all           include the nodes of the largest component (prints all
                        nodes of the graph)
    -h, --help          show this help
```

For more help, see the official [documentation](http://nfleet.github.io/isolation/).

## Installation

You need Rust 1.0 to build this. Install it and then clone this repo and then run `cargo build --release` in the directory. The binary is then available in `target/release/isol`.

## License

Copyright &copy; 2015 [NFleet Oy](http://www.nfleet.fi). All rights reserved.

Licensed under the Modified BSD License (also known as the 3-clause BSD License), see [LICENSE.md](./LICENSE.md) for details.
