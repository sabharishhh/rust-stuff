// using .collect() method

fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let v1_iter = v1.iter().filter(|x| *x % 2 != 0).map(|x| *x + 2);

    let v2: Vec<i32> = v1_iter.collect();
    println!("{:?}", v2);
    
}
