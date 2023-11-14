use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let freqency = frequencies.entry(num).or_insert(0);
        *freqency += 1;
    }

    let mut result = Vec::new();

    for (num, freqency) in frequencies {
        result.push((num, freqency));
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];

    let result = logic(numbers);

    println!("The frequency of each number in the vector is {:?}", result);
}
