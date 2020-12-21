#![deny(warnings)]
#![allow(unused_imports)]

// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

fn main() {}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;
}
