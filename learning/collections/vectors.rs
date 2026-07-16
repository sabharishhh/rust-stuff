// collections are a set of data structures for which RUST provides the template
// collections can contian multiple data value of different types
// data pointed by collections are stored on heap
// vectors are structs themselves as per the template defenition in RUST

// vectors:

 fn main() {
     let mut vec = Vec::new();

     vec.push(1);
     vec.push(3);
     vec.push(2);

     println!("{:?}", vec);

     let new = even_filter(vec);
     println!("{:?}", new);

    // alternative way to initialize a vector:
    let mut vec1 = vec![1, 2, 3];
    vec1.push(0);
 }

 fn even_filter(vec: Vec<i32>) -> Vec<i32> {
     let mut new = Vec::new();

     for val in vec {
         if val % 2 == 0 {
             new.push(val);
         }
     }
     return new;
 }