fn main() {
    stack_fn();
    heap_fn();
    update_string();
}

fn stack_fn() {
    let b: i32 = 20;
    let a: i32 = 10;
    let c: i32 = a + b;

    println!("Stack function: the sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let combined = format!("{} {}", s1, s2);
    
    println!("Heap function: Combined string is {}", combined);
}

fn update_string() {
    let mut s: String = String::from("Initial String");
    println!("Before update: {}", s);
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    let new_s: String = String::from(" and some additional content");
    s.push_str(&new_s);
    println!("After update: {}", s);
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
}

// the rust compiler has two data structures powering the memory, stack containing stack frames and a heap memory
// stack is known to have quick read, write and access speeds. it stotes int, float, pointers like datatypes
// heap is slower but expandable or flexible, it stores all the string, vector, hashmap and large data structures allowing for runtime data accomodation over rigid size boundaries
// each function call creates a stack frame, the variables associated with that fn call is stored in that frame, once the control returns to the caller, the stack frame is popped out.
// a string var is stored in the heap and the pointer to the first byte of the same is stored in a stack frame itself.
// 
// stack is allocated at compile time, static. heap is allocated at runtime, dynamic
// stack is smaller whereas heap has a larger capacity