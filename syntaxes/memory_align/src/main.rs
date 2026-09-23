// 内存对齐规则：
//  1. 系统会有一个默认对齐数 N（通常等于 CPU 字长，常见 x86_64 下就是 8）
//  2. 第一个元素放在偏移量 0。
//  3. 后续元素的对齐数 Si = min(自身大小, N)，该元素的偏移量必须是 Si 的倍数。
//  4. 结构体整体大小也必须是所有字段Si的最大值的倍数。
// 以ExampleC举例：
// 1. 假设默认对齐数 N = 8
// 2. 第一个元素大小为1 byte，S1 = min(1, 8) = 1，由于它是第一个元素，自动对齐到偏移量为0的位置，共占1 byte。
// 3.1 第二个元素大小为2 byte，S2 = min(2, 8) = 2，所以第二个元素必须对齐到偏移量为2的倍数处，在第一个元素填充完毕后，第一个2的倍数处就是2本身。所以它对齐到2，占用2 byte。
// 3.2 第三个元素大小为8 byte，S3 = min(8, 8) = 8，所以第三个元素必须对齐到偏移量为8的倍数处，上一个元素结束后，下一个为8的倍数位置就是8本身。所以它对齐到8，占用8 byte。
// 3.3 第四个元素大小为4 byte，S4 = min(4, 8) = 4，所以第四个元素必须对齐到偏移量为4的倍数处，上一个元素结束后，下一个为4的倍数位置是16本身。所以它对齐到16，占用4 byte。
// 4. 最后取出max(S1, S2, S3, S4)的最大值8，整个结构体的大小还必须是8的倍数，因此在d后面又会多出4 byte的空白，整个结构体大小为24 byte。
#[repr(C)]
struct ExampleC {
    a: u8,  // 1
    b: u16, // 2
    c: u64, // 8
    d: u32, // 4
}

// Rust会根据内存布局，自动重新排列元素，基于内存对齐规则前提下，让结构体占用最小空间
// 实现自动内存优化，这是Rust的默认行为，默认添加了属性：
// #[repr(Rust)] // 默认
#[allow(unused)]
struct ExampleRust {
    a: u8,  // 1
    b: u16, // 2
    c: u64, // 8
    d: u32, // 4
}

// 程序员调整元素顺序实现手动优化
#[repr(C)]
struct ExampleCOpti {
    a: u8,  // 1
    b: u16, // 2
    d: u32, // 4，和c交换位置
    c: u64, // 8
}

// 结构体嵌套
// 当结构体内嵌套结构体，那么内部的结构体初始对齐数为它所有对齐数Si的最大值。
// 内部嵌套的b: Inner 内存对齐规则推导：
// Sx = min(4, 8) = 4，Sy = min(2, 8) = 2
// 当Inner作为一个元素嵌套到别的结构体，它的初始对齐数是S_inner = max(Sx, Sy) = 4
// 随后在对b对齐的时候，S2 = min(S_inner, 8) = 4，即b会对齐到4的倍数处。
#[repr(C)]
struct Inner {
    x: u32,
    y: u16,
}
#[repr(C)]
struct Outer {
    a: u8,
    b: Inner,
    c: u64,
}

// 紧密打包，取消对齐填充
#[repr(packed)]
#[allow(unused)]
struct ExamplePacked {
    a: u8,
    b: u16,
    c: u64,
    d: u32,
}

// 联合体
// 联合体中，多个元素使用同一块内存，其实它的内存策略非常简单，联合体的大小取所有变体中最大的那个值。
#[allow(unused)]
union U {
    a: u32,
    b: u64,
    c: f32,
}

// 对于普通的枚举，它只占1 byte。
// 底层通过一个u8类型的整数，来判别当前是哪一个枚举值，这个数字称为判别值。
#[allow(unused)]
enum Color {
    Red,
    Green,
    Blue,
}

// 枚举携带参数
#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}
// Message枚举相当于以下结构体+联合体（数据）
// 在整个结构体头部存了一个u8的判别值，随后把三个变体放到了同一个union中，最后取内存占用最大的值。
#[allow(unused)]
union MessageData {
    quit: (),
    mov: (i32, i32),
    change_color: (i32, i32, i32),
}
// 在data中有三个元素，quit、mov、change_color，其中mov和change_color可以视为内嵌的结构体
// 它们的对齐数分别为S_mov = max(4, 4) 和S_col = max(4, 4, 4)，这里所有的4都是元组内i32的大小。
// data的对齐数就是S_data = max(0, S_mov, S_col) = 4。因此当data内嵌到MessageMemoryLayout时，要对齐到4。
#[allow(unused)]
struct MessageMemoryLayout {
    discriminant: u8,   // 判别值
    data: MessageData,  // 携带的数据
}

fn main() {
    // 结构体内存对齐
    println!("size_of::<ExampleC>() = {}", size_of::<ExampleC>()); // 24
    println!("size_of::<ExampleRust>() = {}", size_of::<ExampleRust>()); // 16
    println!("size_of::<ExampleCOpti>() = {}", size_of::<ExampleCOpti>()); // 26
    println!("size_of::<Outer>() = {}", size_of::<Outer>()); // 24
    println!("size_of::<ExamplePacked>() = {}", size_of::<ExamplePacked>()); // 15

    // 元组的排布，和结构体是完全相同的，同样遵循内存对齐的策略
    println!("size_of::<(u8, u16, u64, u32) = {}", size_of::<(u8, u16, u64, u32)>()); // 16
    println!("size_of::<(u8, u16, u32, u64)>() = {}", size_of::<(u8, u16, u32, u64)>()); // 16

    // 联合体
    println!("size_of::<U>() = {}", size_of::<U>()); // 8

    // 枚举
    println!("size_of::<Color>() = {}", size_of::<Color>()); // 1
    println!("size_of::<Message>() = {}", size_of::<Message>()); // 16
    println!("size_of::<MessageMemoryLayout>() = {}", size_of::<MessageMemoryLayout>()); // 16
}