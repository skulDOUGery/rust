fn main() {
    let mut vec = Vec::with_capacity(10);

    for i in 0..10 {
        vec.push(i);
    }

    println!("Vector: {:?}, Capacity: {}", vec, vec.capacity());
}
