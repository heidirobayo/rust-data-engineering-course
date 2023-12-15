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
