mod types;
use types::data;

use art::mix;
use art::PrimaryColor;
use art::SecondaryColor;

mod quux {
    pub use self::foo::{bar, baz};

    pub mod foo {
        pub fn bar() {}
        pub fn baz() {}
    }
}

fn main() {
    quux::bar();
    quux::baz();
    let r = mix(PrimaryColor::Blue, PrimaryColor::Red);
    println!("{:#?}", r);
    assert_eq!(SecondaryColor::Orange, r);
    data::foo();
}
