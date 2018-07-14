pub mod vk {
    include!(concat!(env!("OUT_DIR"), "/gen_global.rs"));
}

pub mod vk_mem_alloc;
