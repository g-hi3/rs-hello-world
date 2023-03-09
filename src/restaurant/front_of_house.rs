// This module needs to be public, because it will be accessed by a parent module.
pub mod hosting

mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}