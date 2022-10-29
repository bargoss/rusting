pub mod factory_experiment_mod {
    // do json serialization
    use serde::{Deserialize, Serialize};


    #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
    struct Foo {
        a: i32,
        b: i32,
    }



    pub fn factory_experiment_main() {
        let a = Foo {a:1,b:2};
        let b = Foo {a:3,b:4};

        // json serialize
        let a_json = serde_json::to_string(&a).unwrap();

        println!("a_json: {}", a_json);




    }
}
