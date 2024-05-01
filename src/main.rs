struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { curr: 1, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        let curr = self.curr;
        self.curr = self.next;
        self.next = new_next;
        Some(curr)
    }
}

fn main(){
    let mut fib = Fibonacci{curr: 0, next: 1};
    for _ in 0..100 {
        println!("{}", fib.next().unwrap());
    }
}
// use std::ops::Add;

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 1, y: 0 };
//     let p2 = Point { x: 2, y: 3 };
//     let p3 = p1 + p2;
//     println!("{} {}", p3.x, p3.y);
// }

// // trait Greeter {
// //     fn greet(&self) -> String;{
// //         // This is the default implementation
// //         "Hello".to_string()
// //     }
// //     fn greet_with_name(&self, name: &str) -> String {
// //         // This is the default implementation
// //         format!("{}, {}!", self.greet(), name)
// //     }

// // }

// // struct Person {
// //     name: String,
// // }

// // impl Greeter for Person {
// //     fn greet(&self) -> String {
// //         // This is the implementation for Person
// //         format!("Hi, I'm {}", self.name)
// //     }
// // }

// // fn main() {
// //     let alice = Person { name: "Alice".to_string() };
// //     println!("{}", alice.greet());
// //     println!("{}", alice.greet_with_name("Bob"));
// // }
