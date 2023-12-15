// create a function to containt all what main function contains
// references: https://doc.rust-lang.org/std/vec/struct.Vec.html

use std::vec;

pub fn print_collections() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");
}

// create a function to test the vector
pub fn test_vect() {
    // examples of vector initialization
    let mut vec = vec![1, 2, 3];
    vec.push(4);

    let vec2 = Vec::from([1, 2, 3, 4]);

    assert_eq!(vec, vec2);

    let vec3 = vec![0; 5];
    assert_eq!(vec3, [0, 0, 0, 0, 0]);

    let mut vec4 = Vec::with_capacity(5);
    vec4 = vec![0; 5];
    assert_eq!(vec4, [0, 0, 0, 0, 0]);
}

// create a function to retains only the elements specified by the predicate.
pub fn test_retain() {
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    vec.retain(|&x| x % 2 == 0);
    assert_eq!(vec, [2, 4, 6]);
    println!("test_retain passed!");
}

/*
 * Because the elements are visited exactly once in the original order, external state may be used to decide which elements to keep.
 * Create a function to keep elements
 */

pub fn test_keep() {
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    let keep = [false, true, true, false, true, false];
    let mut keep_iter = keep.iter();
    vec.retain(|_| *keep_iter.next().unwrap());
    assert_eq!(vec, [2, 3, 5]);
    println!("test_keep passed!");
}

/*
 * Retains only the elements specified by the predicate, passing a mutable reference to it.
 * In other words, remove all elements e such that f(&mut e) returns false. This method operates in place,
 * visiting each element exactly once in the original order, and preserves the order of the retained elements.
*/
pub fn test_retain_according_to_predicate() {
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    vec.retain_mut(|x| {
        if *x <= 4 {
            *x += 1;
            true
        } else {
            false
        }
    });

    assert_eq!(vec, [2, 3, 4, 5]);
    println!("test_retain_according_to_predicate passed!");
}
