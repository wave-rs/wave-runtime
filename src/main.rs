
extern "C" {
    pub fn wamr_runtime_init();
}
fn main() {
 
    println!("Hello, Wave Runtime!");
    unsafe { wamr_runtime_init() };
}
