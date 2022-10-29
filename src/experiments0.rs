pub mod experiments0_stuff{
    struct SerializableComponent<T>{
        data: T
    }
    impl<T> SerializableComponent<T>{
        fn new(data: T) -> Self{
            Self{
                data
            }
        }
    }

    pub fn main() {

    }
}