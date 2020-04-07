use side_lib;
fn main() {
    better_panic::install();

    panic!("hi");
}
