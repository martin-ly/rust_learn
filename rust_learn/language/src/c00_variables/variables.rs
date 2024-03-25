/*
    Binding and mutability
        1. A variable can be used only if it has been initialized.
        2. Use mut to mark a variable as mutable.
  
    Scope
        A scope is the range within the program for which the item is valid.

    Shadowing
        You can declare a new variable with the same name as a previous variable,
        here we can say the first one is shadowed by the second one.

    Unused variables

    Destructuring
        We can use a pattern with let to destructure a tuple to separate variables.
    
    Destructuring assignments
        
*/



// Global variable with 'static lifetime, single memory location.
static X: T = T();

// Defines constant, copied into a temporary when used.
const X: T = T();

fn test() -> !{
    // Allocate T bytes on stack bound as x . Assignable once, not mutable.
    let x: T;

    // Like let , but allow for mutability and mutable borrow.
    let mut x: T;

    // Moves y to x , inval. y if T is not Copy , and copying y otherwise.
    x = y
}

