

// Define an enum E, algebraic data types, tagged unions.
enum E0 {}

//Define variants of enum; can be unit- A , tuple- B () and struct-like C{} .
enum E1 {
    A,
    B(),
    c{}
}

// If variants are only unit-like, allow discriminant values, e.g., for FFI.
enum E2 {
    A = 1
}

// Enum w/o variants is uninhabited, can't be created, c. 'never'
enum E3 {

}

