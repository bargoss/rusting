pub mod factory_experiment_mod {
    // use serde
    pub use serde::{Serialize, Deserialize};

    // objects that are created from the factory:
    pub struct School {
        pub id: i32,
        pub name: String,
        pub address: String
    }
    pub struct Student {
        pub id: i32,
        pub name: String,
        pub age: u8,
        pub schoolId: i32
    }
    pub struct Book {
        pub id: i32,
        pub name: String,
        pub author: String,
        pub pages: u16
    }
    pub struct BookBorrow {
        pub id: i32,
        pub studentId: i32,
        pub bookId: i32,
        pub borrowDate: String,
        pub returnDate: String
    }

    // serializable point with serde
    #[derive(Serialize, Deserialize)]
    pub struct SchoolJson {
        pub id: i32,
        pub name: String,
        pub address: String
    }







    pub fn factory_experiment_main(){
        println!("Factory Experiment");
    }
}
