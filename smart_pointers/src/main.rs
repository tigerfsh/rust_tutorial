use std::ops::Deref;

fn main() {
    // Box 
    let a = Box::new(5);
    println!("a: {a}");

    // Deref
    // let a = 5;
    // let b = &a;
    let a = 5;
    let b = Box::new(a);
    
    if a == *b {
        println!("Equal");
    } else {
        println!("Not equal");
    }

    let a = 5;
    let b = CustomBox::new(a);
    if a == *b {
        println!("Equal");
    } else {
        println!("Not equal.");
    }

    // Drop 
    let _ = Example{a: 1};
    let _ = Example{a:5};
    
}

struct CustomBox<T>(T);

impl<T> CustomBox<T> {
    fn new(y: T) -> CustomBox<T> {
        CustomBox(y)
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Example {
    a: i32
}

impl Drop for Example {
    fn drop(&mut self) {
        println!("Dropping the example, a: {}", self.a);
    }
}