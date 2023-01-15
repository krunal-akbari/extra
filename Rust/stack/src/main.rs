use crate::builtin::operations::basic;
pub mod builtin;

fn main() {
    let mut q = basic::create_stack();
    let name = String::from("a+b-(c*(d+e)/f)");
    //let name = String::from("a+b");
    basic::string_to_stuct(&mut q, name);
    let ans = basic::postfix(&mut q);
    ans.display();
}
