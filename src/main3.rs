// Let's create numbers be a vector from 1-5
//let numbers = vec![1, 2, 3, 4, 5,6,7,8,9,10];
//let sum = numbers.iter().sum();

//vector implements iterator. We bring the iterator

//Lets use trait for soemthing Printable

trait Printable {
    fn print(&self);
}

struct Circle {
    radius: f64,
}

impl Printable for Circle {
    fn print(&self) {
        println!("Circle of radius {}", self.radius);
    }
}

// fn main(){
//     let numbers = vec![1, 2, 3, 4, 5,6,7,8,9,10];
//     let sum:i32 = numbers.iter().sum();

   
//     println!("The sum of the numbers is: {}", sum);
// }

// fn main(){
//     let c = Circle { radius: 3.0 };
//     c.print();
// }

trait Drawable {
    fn draw(&self);
        // Everything that implements draw will do this
}

struct Circle {
    radius: f64,
}   

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle of radius {}", self.radius);
    }
}


struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle of width {} and height {}", self.width, self.height);
    }
}

fn draw_shapes<T: Drawable>(shapes: &[T]) {
    for shape in shapes {
        shape.draw();
    }
}

fn main(){
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 2.0, height: 5.0 }),
        Box::new(Circle { radius: 2.5 }),
    ];
    draw_shapes(&shape.unwrap());
}