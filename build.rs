extern crate cc;
extern crate vk_generator;
extern crate vk_api;
extern crate bindgen;

use std::env;
use std::path::Path;
use std::fs::{File, DirBuilder};

use vk_generator::{VkVersion, GenConfig};
use vk_generator::generator::GenRegistry;

fn main() {
    cc::Build::new()
        .include("vma")
        .file("vma/vma.cpp")
        .cpp(true)
        .define("VMA_STATIC_VULKAN_FUNCTIONS", Some("0"))
        .compile("vk_mem_alloc");

    let out = env::var("OUT_DIR").unwrap();
    DirBuilder::new().recursive(true).create(&out).unwrap();

    let mut file = File::create(&Path::new(&out).join("gen_global.rs")).unwrap();
    let registry = vk_generator::VkRegistry::new(vk_api::VK_XML);
    registry.gen_global(
        &mut file,
        VkVersion(1, 0),
        &[
            "VK_KHR_surface",
            "VK_EXT_validation_cache",
            "VK_KHR_get_memory_requirements2",
            "VK_KHR_dedicated_allocation",
            "VK_EXT_sample_locations",
            "VK_KHR_get_physical_device_properties2",
            "VK_KHR_bind_memory2",
        ],
        Default::default()
    );

    // let mut bindings_builder = bindgen::builder()
    //     .blacklist_type("Vk.*")
    //     .default_enum_style(bindgen::EnumVariation::Rust)
    //     .bitfield_enum(".*Bits$")
    //     .header("vma/vk_mem_alloc.h");

    // use std::io::Write;
    // let mut out = File::create("out").unwrap();
    // // for type_name in registry.types().keys() {
    // //     bindings_builder = bindings_builder.blacklist_type(type_name);
    // //     writeln!(out, "{}", type_name);
    // // }

    // let bindings = bindings_builder.generate().unwrap();
    // bindings.write_to_file("./src/vk_mem_alloc_new.rs").unwrap();
}
