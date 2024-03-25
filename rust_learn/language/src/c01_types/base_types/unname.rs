/// These sigils did not fit any other category but are good to know nonetheless.
/// 
/// 
/// 
/// 


//! test 
fn f1() {
    let x = 1_u8;
    
    let _ = 123_u16;

    // You can assign anything to _ without let
    _ = x;

    // Variable binding that won't emit unused variable warnings.
    let _warn = 123_123_789_u32;

    // Numeric separator for visual clarity.
    let _num = 1_123_456_789 ;

    // Type specifier for numeric literals
    let _type = 1_i64;

    // Hexadecimal (0x ), octal (0o ) and binary (0b ) integer literals.

    let _Hex = 0xBEEF;
    let _Octal = 0o777;
    let _Bin = 0b0101;

}