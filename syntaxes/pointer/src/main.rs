mod r#box;
use crate::r#box::box_pointer_test;

mod rc;
use crate::rc::rc_pointer_test;

mod refcell;
mod mix;
use crate::mix::mixed::rc_refcell_pointer_test;

mod r#loop;
use crate::r#loop::rc_refcell_loop_test;

mod weak;
use crate::weak::weak_pointer_test;

fn main() {
    box_pointer_test();
    rc_pointer_test();
    rc_refcell_pointer_test();
    rc_refcell_loop_test();
    weak_pointer_test();
}
