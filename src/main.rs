use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;

trait Data {
    fn as_any_ref(&self) -> &dyn Any;
}

impl<T> Data for T
where
    T: Any,
{
    // This cast cannot be written in a default implementation so cannot be
    // moved to the original trait without implementing it for every type.
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct Foo;

#[derive(Debug)]
struct Bar;

fn main() {
    let mut v: Vec<Box<dyn Data>> = vec![];

    v.push(Box::new(Foo));
    // v.push(Box::new(2u32));

    for d in v {
        // let d = d.deref();
        let d = d.deref().as_any_ref().downcast_ref::<Foo>();
        match d {
            Some(f) => println!("{:?}", f),
            None => println!("No Foo..."),
        };

        // match x.downcast_ref::<u32>() {
        //     Some(b) => println!("word {:?}", b),
        //     None => println!("No word..."),
        // };
    }
}

// fn log(value: &dyn Any) {
//     match value.downcast_ref::<String>() {
//         Some(text) => println!("Bytes of the string: {:?}", text.as_bytes()),
//         None => println!("No string..."),
//     };
// }

// fn main() {
//     // &T is coerced into &dyn Any:
//     log(&String::from("hello"));
//     log(&10);
// }

// trait MyTrait: Any {}
// #[derive(Clone)]
// struct Foo {}
// #[derive(Clone)]
// struct Bar {}

// impl MyTrait for Foo {}

// impl Foo {
//     // fn downcast(&self) -> Option<Self> {
//     //     match <dyn Any>::downcast_ref::<Foo>(self) {
//     //         Some(s) => Some(s.clone()),
//     //         None => None,
//     //     }
//     // }
// }

// impl MyTrait for Bar {}

// impl Bar {}

// struct Storage {
//     data: Vec<Box<dyn MyTrait>>,
// }

// fn main() {
//     let s = Storage {
//         data: vec![Box::new(Bar {})],
//     };
//     let first_element = match <dyn Any>::downcast_ref::<Bar>(&s.data[0]) {
//         Some(s) => {
//             println!("some");
//             Some(s.clone())
//         }
//         None => None,
//     }; // compile error

//     println!("here");
// }

// use std::any::{Any, TypeId};
// use std::mem::size_of;
// use std::mem::transmute;

// trait Data {
//     fn try_get<T: Any>(&self) -> Option<&T>
//     where
//         Self: Any, //  + Sized,
//     {
//         // println!("T: {:?}", TypeId::of::<T>());
//         // println!("Self: {:?}", TypeId::of::<Self>());

//         // if TypeId::of::<T>() == TypeId::of::<Self>() {
//         // if size_of::<T>() == size_of::<Self>() {
//         <dyn Any>::downcast_ref::<T>(&self)
//         // } else {
//         //     None // unreachable!()
//         // }
//         // } else {
//         //     None
//         // }
//     }
// }

// // impl<D: Any> Data for D {}

// fn main() {
//     let mut v: Vec<Box<dyn Data>> = Vec::new();
//     // v.push(Box::new(72u32));
//     // v.push(Box::new(32u8));

//     // for d in v {
//     //     println!("u32 {:?}", d.try_get::<u32>());
//     //     println!("u8 {:?}", d.try_get::<u8>());
//     // }
// }
