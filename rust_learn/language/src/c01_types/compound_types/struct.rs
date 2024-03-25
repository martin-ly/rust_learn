
///! Data types and memory locations defined via keywords.


// Define a struct with named fields.
struct S0 {
    n1 : int,
    n2 : string
}

// Define zero sized unit struct. Occupies no space, optimized away.
struct S1; 

// Define "tupled" struct with numbered field .0 of type T.
#[derive(Debug)]
struct S2(int); 

