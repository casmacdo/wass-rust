use wasm_bindgen::prelude::*; // this imports the wasm_bingen::prelude module. 
//wasm-bindgen is a tool used by wasm-pack that connects js and rust. In effect, it allows js to call a rust api with a string or a rust function to catch a js exception 
#[wasm_bindgen] // this is an attribute that modifies the next statement
extern { // extern is a statement that tells rust that we want to call externally defined functions. The attribute above states that wasm-bindgen knows how to find these functions
    pub fn alert(s: &str); // this is a function signature the js alert function that takes one string argument. The pub key allows access beyond the module's scope

}
// next we produce a rust function that js can call

// again, we use the wasm_bindgen attribute, which modifies the next statement 
#[wasm_bindgen]
// the pub key allows access beyond the module's scope
// fn declares a function named greet that uses a string argument called name. Then it calls the alert function which we brought in above and passes a call to the format macro, which concatenates strings
pub fn greet(name: &str) { // declair function, argument, and variable type 
    alert(&format!("Hello, {}!", name)); // declare macro
}