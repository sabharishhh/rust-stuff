// iterator adaptors are methods defined on the iterator trait that dont consume the iterator
// Instead, they produce different iterators by changing some aspect of the original iterator

fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let v1_iter = v1.iter();

    let v1_filter = v1_iter.filter(|x| *x % 2 != 0);
    let v1_double = v1_filter.map(|x| *x + 2);

    for i in v1_double {
        println!("{}", i);
    }

    println!("{:?}", v1);
}