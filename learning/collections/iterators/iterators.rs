// iterator pattern allows to perform some task on a sequence of items, turn-by-turn.
// responsible for the logic of iterating over each item and determining when a seq has finished
// rust iterators are lazy, they have no effect until methods that consume the iterator itself are called

fn main() {
    let mut nums = vec![1, 2, 3];
    let mut vec_iter = nums.iter_mut(); // iter is only a borrower and doesnt become the owner of the values pointed by 'nums' vector

    while let Some(val) = vec_iter.next() { // after consuming the last item ie., 3 in this case the next data item is None for which the while condition fails and stops iterating
        *val += 10; // * is the dereference operator. val is not the integer, it is the mutable ref to an integer (val: &mut i32)
        println!("{}", val);
    }

    println!("{:?}", nums);

    // for val in vec_iter {
    //     *val += 1;  // since iter is a only a borrower, pass by ref is required  
    //     println!("{}", val);
    // }

    // into_iter() takes ownership of the collection set.
    // 
}

