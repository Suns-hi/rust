// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-compare-mode-nll

#![feature(box_syntax)]

fn take(_v: Box<isize>) {
}

fn box_imm() {
    let v = box 3;
    let _w = &v;
    take(v); //~ ERROR cannot move out of `v` because it is borrowed
}

fn main() {
}
