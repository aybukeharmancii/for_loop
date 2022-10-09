use std::vec;

fn main() {
    let animals = vec!["Rabbit", "Dog", "Cat"];
    let numbers = 30..51;

    // for i in 1..11 {
    //     println!("The number is {}", i);
    // }

    // for i in numbers {
    //     println!("The number is {}", i);
    // }

    for (index,a) in animals.iter().enumerate() {
        println!("The index is {} and the animal is {}", index, a);
    }
}
