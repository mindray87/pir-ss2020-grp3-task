use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main () {
    // To get started with closures, why dont you give it a try and create 
    // two closures: One to increment a given integervalue and one to decrement it.
    // whilest on this rather simple task, try to make the increment closure fully inferred! 

    // let increment_closure = ...
    // let decrement_closure = ...

    assert_eq!(increment_closure(5), 6);
    assert_eq!(decrement_closure(5), 4);

    // Rebuild this function to a closure. 

    fn get_employees() -> HashMap<String, String> {
        println!("Get users from massive DB, this might take a while");
        thread::sleep(Duration::from_secs(2));
        let mut employees = HashMap::new();
        employees.insert(
            "Hans Müller".to_string(),
            "350000".to_string()
        );
        employees.insert(
            "Peter Pan".to_string(),
            "90000".to_string()
        );
        employees.insert(
            "Sandy McSanderson".to_string(),
            "500000".to_string()
        );
        employees.insert(
            "Bibi Bloxberg".to_string(),
            "150000".to_string()
        );
        employees
    }

    // call with closure
    let worker_with_salary = get_employees();
    
    // Now  find the worker with the highest income. Use an iterator

    //let highest_salary = ...
    assert_eq!(highest_salary, "Sandy McSanderson".to_string());

    // now because we think Hans and Peter earn to less, we want to manipulate the data we 
    // get from get_employees, so that they both get 750000. 
    // again use an iterator to manipulate the elements 
    let worker_with_salary = get_employees();
    // let hans = ...
    assert_eq!(hans, "750000".to_string);
    // now write a closure that returns the value for a given key using another call to 
    // the get_employee closure (which by now should return )
    // let get_salary_by_id || = ..
    assert_eq!(get_salary_by_id("Hans Müller".to_string()), "350000".to_string());

    // so far we have made three calls to out get employees closure. 
    // As this simulates a rather expensive call, we want to build a Cacher struct,
    // that holds the closure and the value. 
    // Implement the Cacher struct, so that we call the closure only once and 
    // ensure, that we always have the given list stored in the value field of the struct.
    // If you run your main, 
    // println!("Get users from massive DB, this might take a while");
    // should only be shown once while the assert_eq tests should still be running.

    // skeleton for Cacher struct and impl: 
    struct Cacher<T>
    where
        T: Fn(...) -> ...,
    {
        getter: T,
        value: HashMap<String,String>
    }

    impl<T> Cacher <T> 
    where 
        T: Fn(...) -> ...,
    {
        fn new(getter: T) -> Cacher<T>{
            Cacher{
                getter,
                value: HashMap::new(),
            }
        }
        fn value(&mut self, arg: String) -> ...{}
    }

}
