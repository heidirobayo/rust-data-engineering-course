/**
 * this program creates a fruit salad by scrambling (shuffling) a list of fruits.
 * A vector is a growable array. It can grow or shirink in size and is one of the most
 * useful data structures is represented using the Vect<T> type.
 */
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruits = vec![
        "Apple",
        "Banana",
        "Cherry",
        "Orange",
        "Pear",
        "Pineapple",
        "Strawberry",
    ];

    // print the original fruits salad
    println!("Fruit salad: {:?}", fruits);

    // shuffle the fruits
    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    for (i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("\n {}, ", item);
        } else {
            print!("\n and {}", item);
        }

        // println!("\n {}. {}", i + 1, item);
    }
}
