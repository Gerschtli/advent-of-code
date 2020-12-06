// necessary for intellij support
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

mod group;
mod person;

fn main() {}

#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::*;
}
