mod example;
mod foo1;
mod foo2;
mod core;

use foo1::{hello::*, test};
use foo2::hello::module_hello as f2_hello;
use foo2::test as f2_test;
use example::*;

fn main() {
    module_hello::call_module_hello();
    test::call_module_test();
    module_example::call_module_example();
    f2_hello::call_module_hello();
    f2_test::call_module_test();
    core::call_from_core();
}
