/******************************************************************************
Rules of mutable and immutable references

1. Multiple immutable references are allowed
2. Only ONE muttable reference is allowed
3. No mixed immutable reference and muttable reference
*******************************************************************************/
fn main() {
    let mut i = 10;
    let p = &i;
    let q = &i;
    //let r = &mut i;
    //let f = &mut i;

    //println!("{}", q);
    //println!("{}", p);
    println!("{}", q);
    println!("{}", p);
}

