/// # Characteristics of Object-Oriented
/// there is no consensus in the programming community about features programming language required to be considered as object oriented
/// in rust structs and enums holds data, and we can use impl to provide methods to a struct and enum
fn main() {
    /* Encapsulation */
    // which means implementaion details are hidden from the code using that object
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }
    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }
        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }
        pub fn average(&self) -> f64 {
            self.average
        }
        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }

    /* Inheritance */
    // it is the ability of an object to inherit data and behaviours of other object
    // in rust we can accomplish this using default trait method implementaion
    // but there is a limitaion, as of now traits can only defind method and not fields
    // one of the reason to use inheritance is polymorphisim
}
