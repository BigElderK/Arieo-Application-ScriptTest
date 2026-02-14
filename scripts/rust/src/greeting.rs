// Generate bindings for the application world
wit_bindgen::generate!({
    world: "application",
    path: "../wit",
    generate_all
});

use crate::arieo::application::host;
use crate::arieo::module::module_manager;
use crate::exports::arieo::application::guest::Guest;

#[path = "sample.wasm.rs"]
mod wrapper;
use wrapper::interface;

// Define our component struct
struct GreeterComponent;

// Implement the guest interface
impl Guest for GreeterComponent {
    fn greeting() -> String {
        // Use the imported host function
        host::log("Rust guest Greeting() called.");
        
        // Call module manager create-instance to get i_sample instance - directly convert to wrapper
        let sample_inst: interface::sample::ISample = module_manager::get_interface(
            interface::sample::ISample::INTERFACE_ID,
            interface::sample::ISample::INTERFACE_CHECKSUM,
            ""
        ).into();
        
        let result1 = sample_inst.do_somthing_1(10, 20);
        host::log(&format!("do-somthing1(10, 20) = {}", result1));
        
        let result2 = sample_inst.do_somthing_2(5, 15);
        host::log(&format!("do-somthing2(5, 15) = {}", result2));
        
        format!("Hello from Rust! Results: {}, {}", result1, result2)
    }
}

// Export the implementation so the component's `greeting` function is emitted
export!(GreeterComponent);