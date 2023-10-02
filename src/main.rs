use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

pub type Id = String;

#[typetag::serde]
trait Component {
    fn get_port(&self) -> (Id, Ports);
}

trait Data {
    fn as_any_ref(&self) -> &dyn Any;
}

impl<T> Data for T
where
    T: Any + Serialize,
{
    // This cast cannot be written in a default implementation so cannot be
    // moved to the original trait without implementing it for every type.
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

pub struct Ports {
    pub inputs: Vec<Input>,
    pub output: Vec<Output>,
}

pub struct Input {
    pub id: Id,
    pub field: Id,
}

pub struct Output {
    pub field: Id,
    data: Rc<dyn Data>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Foo {
    id: Id,
    out: (Id, Rc<RwLock<u32>>),
}

#[typetag::serde]
impl Component for Foo {
    fn get_port(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![],
                output: vec![Output {
                    field: self.out.0.clone(),
                    data: self.out.1.clone(),
                }],
            },
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Bar {
    id: Id,
    out: (Id, Rc<RwLock<u8>>),
}

#[typetag::serde]
impl Component for Bar {
    fn get_port(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![],
                output: vec![Output {
                    field: self.out.0.clone(),
                    data: self.out.1.clone(),
                }],
            },
        )
    }
}

fn main() {
    type Components = Vec<Rc<dyn Component>>;
    let mut v: Components = vec![];

    let foo = Rc::new(Foo {
        id: "foo".into(),
        out: ("out".into(), Rc::new(RwLock::new(22))),
    });

    let bar = Rc::new(Bar {
        id: "bar".into(),
        out: ("out".into(), Rc::new(RwLock::new(33))),
    });

    v.push(foo);
    v.push(bar);

    let mut id_ports = std::collections::HashMap::new();
    for i in v {
        let (id, ports) = i.get_port();
        id_ports.insert(id, ports);
    }

    for (id, port) in id_ports {
        println!("id {}", id);
        match port.output[0]
            .data
            .deref()
            .as_any_ref()
            .downcast_ref::<u8>()
        {
            Some(f) => println!("u8 {:?}", f),
            None => println!("No u8.."),
        };

        println!("id {}", id);
        match port.output[0]
            .data
            .deref()
            .as_any_ref()
            .downcast_ref::<RwLock<u32>>()
        {
            Some(f) => println!("u32 {:?}", f),
            None => println!("No u32.."),
        };
    }

    // v.push(Box::new(42u8));
    // v.push(Box::new(1337u32));

    // for d in v {
    //     let d = d.deref().as_any_ref();

    //     match d.downcast_ref::<u8>() {
    //         Some(f) => println!("u8 {:?}", f),
    //         None => println!("No u8.."),
    //     };
    //     match d.downcast_ref::<u32>() {
    //         Some(f) => println!("u32 {:?}", f),
    //         None => println!("No u32.."),
    //     };
    // }

    // let mut v: Vec<Box<dyn Component>> = vec![];

    // v.push(Box::new(Foo));
    // // v.push(Box::new(2u32));

    // for c in v {
    //     let p = c.as_any_ref();
    //     let d = p.downcast_ref::<Foo>();
    //     match d {
    //         Some(f) => println!("{:?}", f),
    //         None => println!("No Foo..."),
    //     };

    //     // match x.downcast_ref::<u32>() {
    //     //     Some(b) => println!("word {:?}", b),
    //     //     None => println!("No word..."),
    //     // };
    // }
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
