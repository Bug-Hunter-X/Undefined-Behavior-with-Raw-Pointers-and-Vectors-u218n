fn main() {
    let mut v = vec![1, 2, 3];
    let len = v.len();
    let ptr = v.as_mut_ptr();

    unsafe {
        for i in 0..len {
            *ptr.add(i) = (*ptr.add(i)) * 2; //modify value
        }
    }

    println!( "{:?}", v);
}