//! **isolation** is a small library for computing strongly connected components of
//! a graph that has been serialized in some format. It uses the [**petgraph**](http://github.com/bluss/petulant-avenger-graphlibrary)
//! library for computing the components. It should be quite fast. See the docs
//! for the [**compute**](./isolation/fn.compute.html) to see the format.
extern crate petgraph;
#[macro_use] extern crate scan_fmt;

pub mod isolation;
