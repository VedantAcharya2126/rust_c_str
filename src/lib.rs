use std::error::Error;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

//Rust bindings to the C functions
unsafe extern "C" {
    /**
    * Function
    *
    * @param *const c_char
    * 			*const c_char : a pointer to a const (read-only) C-style string
    * @return
    *           *mut c_char : a pointer to the new, dynamically allocated string
    * @author
    *          Vedant Acharya
    */
    fn concat_string(name: *const c_char) -> *mut c_char;

    /**
    * This function deallocates the memory that was previously allocated by malloc() within the concat_string function.
    *
    * @param *mut c_char
    * 			 It takes a single argument, c_char, a pointer to a string.
    */
    fn free_string(s: *mut c_char);
}

/**
 * Function
 *
 * @param data
 * 			No Parameter
 * @return
 *           Result<(), Box<dyn Error>>
 * @author
 *          Vedant Acharya
 */
pub fn concate_str_c_r(name: &str) -> Result<String, Box<dyn Error>> {
    
    let rust_string = name;
    
    // Converting Rust string to a CString (null-terminated).
    let r_convt_str_c: CString = CString::new(rust_string)?;

    let rust_result: String;
    
    unsafe {
        // Pass the CString's raw pointer to the C function.
        let c_string = concat_string(r_convt_str_c.as_ptr());

        // Check for a null pointer, which indicates a C allocation error.
        if c_string.is_null() {
            return Err("C function failed to allocate memory".into());
        }

        // Convert the C result pointer into a CStr.
        let c_result_str = CStr::from_ptr(c_string);

        // Convert the CStr to a Rust String. This performs a memory copy.
        rust_result = c_result_str.to_string_lossy().into_owned();

        // 5. Explicitly free the memory allocated by the C function.
        free_string(c_string);
    }

    println!("The concatenated string from C is: {}", rust_result);
    Ok(rust_result)

}
//Compile library files.
//gcc -c concat-string.c -o concat-string.o

//Create static library. This step is to bundle multiple object files in one static library
//ar rcs lib/libconcat-string.a src/concat-string.o 

//.o files are objects. They are the output of the compiler and input to the linker/librarian.
//.a files are archives. They are groups of objects or static libraries and are also input into the linker.
