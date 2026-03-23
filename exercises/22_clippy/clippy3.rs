// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(value) = my_option {
        println!("{:?}", value);
    }

    let my_arr = &[
        -1, -2, -2,
        -4, -1, -6,
    ];
    println!("My array! Here it is: {my_arr:?}");

    let my_empty_vec = ();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
