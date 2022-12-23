fn main() {
    let mut b = freertos_cargo_build::Builder::new();

    // Path to FreeRTOS kernel or set ENV "FREERTOS_SRC" instead
    b.freertos("../common/FreeRTOS-Kernel");
    // Location of `FreeRTOSConfig.h`
    b.freertos_config("src");
    // Port dir relative to 'FreeRTOS-Kernel/portable'
    b.freertos_port(String::from("GCC/ARM_CM4F"));
    // Set the heap_?.c allocator to use from 'FreeRTOS-Kernel/portable/MemMang' (Default: heap_4.c)
    // b.heap(String::from("heap4.c"));   

    // b.get_cc().file("More.c");   // Optional additional C-Code to be compiled

    b.compile().unwrap_or_else(|e| { panic!("{e}") });
}