use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruit: Vec<&str> = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    let mut rng: ThreadRng = thread_rng();

    fruit.shuffle(&mut rng);

    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
