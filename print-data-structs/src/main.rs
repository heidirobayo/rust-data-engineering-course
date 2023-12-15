mod voperations;
use voperations::vec_operations;
fn main() {
    vec_operations::print_collections();
    vec_operations::test_vect();
    vec_operations::test_retain();
    vec_operations::test_keep();
    vec_operations::test_retain_according_to_predicate();
}
