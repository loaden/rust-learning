fn main() {
    let f = IntOrFloat::from_float(3.14);
    let i = IntOrFloat::from_int(5);
    unsafe {
        println!("{}, {}", f.as_float(), i.as_int());
        println!("{}, {}", f.as_int(), i.as_float());
    }
}

#[repr(C)]
union IntOrFloat {
    i: i32,
    f: f32,
}

impl IntOrFloat {
    // 创建整数变体
    fn from_int(i: i32) -> Self {
        IntOrFloat { i }
    }

    // 创建浮点数变体
    fn from_float(f: f32) -> Self {
        IntOrFloat { f }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn as_int(&self) -> i32 {
        self.i
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn as_float(&self) -> f32 {
        self.f
    }
}
