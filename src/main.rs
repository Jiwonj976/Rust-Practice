**************************************************************************
1. Ownership and Scope

*******************************************************************************/
struct CrabString(String);

impl Drop for CrabString {
    // drop function will be called when CrabString object is being destroyed
    fn drop(&mut self) {
        println!("oh no, my value: [{}] is being destroyed", self.0);
    }
}

fn main() {
    println!("PROGRAM STARTED");
    {
        let _s = CrabString(String::from("My name is ferris"));
    }
    println!("PROGRAM ENDED");
}

