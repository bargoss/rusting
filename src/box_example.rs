/*
Box, stack and heap
All values in Rust are stack allocated by default. Values can be boxed (allocated on the heap) by creating a Box<T>.
A box is a smart pointer to a heap allocated value of type T.
When a box goes out of scope, its destructor is called, the inner object is destroyed, and the memory on the heap is freed.

Boxed values can be dereferenced using the * operator; this removes one layer of indirection.
 */

pub mod box_example {
    use std::mem;

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    // A Rectangle can be specified by where its top left and bottom right
// corners are in space
    #[allow(dead_code )]
    #[derive (Clone, Copy)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn boxed_origin() -> Box<Point> {
        // Allocate this point on the heap, and return a pointer to it
        Box::new(Point { x: 0.0, y: 0.0 })
    }

    pub fn main(){

        /*
         A side note:
            let mut s = String::from("robust & precise")

            let s1 = &s; // no problem
            let s2 = &s; // no problem

            let s3 = mut &s // problem! damaged good bc s1 and s2 are already accessing

            println!("{} {}", s1, s2);
            let s4 = &mut s; // this time: no problem bc s1 & and s2 wonT be used after println! (Compiler checks out)

         */
        let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
            top_left: origin(),
            bottom_right: Point { x: 3.0, y: -4.0 },
        });
        //let boxed_rectangle2 = boxed_rectangle.clone();
        //boxed_rectangle.bottom_right;

        let rect0 = Rectangle {
            top_left: origin(),
            bottom_right: Point { x: 3.0, y: -4.0 },
        };
        let rect1 = rect0.clone();
        // slet rect1 = rect0.clone();

        println!("rect0: {}", rect0.bottom_right.x);
    }

    pub fn main2() {
        // (all the type annotations are superfluous)
        // Stack allocated variables
        let point: Point = origin();
        let rectangle: Rectangle = Rectangle {
            top_left: origin(),
            bottom_right: Point { x: 3.0, y: -4.0 }
        };

        // Heap allocated rectangle
        let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
            top_left: origin(),
            bottom_right: Point { x: 3.0, y: -4.0 },
        });

        // The output of functions can be boxed
        let boxed_point: Box<Point> = Box::new(origin());

        // Double indirection
        let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

        println!("Point occupies {} bytes on the stack",
                 mem::size_of_val(&point));
        println!("Rectangle occupies {} bytes on the stack",
                 mem::size_of_val(&rectangle));

        // box size == pointer size
        println!("Boxed point occupies {} bytes on the heap",
                 mem::size_of_val(&boxed_point));
        println!("Boxed rectangle occupies {} bytes on the heap",
                 mem::size_of_val(&boxed_rectangle));
        println!("Boxed box occupies {} bytes on the heap",
                 mem::size_of_val(&box_in_a_box));

        // Copy the data contained in `boxed_point` into `unboxed_point`
        let unboxed_point: Point = *boxed_point;
        println!("Unboxed point occupies {} bytes on the stack",
                 mem::size_of_val(&unboxed_point));
    }
}