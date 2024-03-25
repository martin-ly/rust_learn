///
//

//! ! Always empty never type.

// Function that never ret.; compat. with any ty. e.g., let x: u8 = f();
fn f()->!{

}

fn f1()->!{
    let x : u8 = f();
}


//! Function that must return Result 
//! but signals it can never Err.
fn f2()->Result<(),!>{

}

//! Function that exists, 
//! but can never be called. Not very useful.
fn f3( x:! ) {

}


