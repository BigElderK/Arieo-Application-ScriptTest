// This file is auto-generated. Do not edit manually.
// Generated from xxx.wasm.rs.mustache template.
// WASM Rust wrapper for interface: arieo:sample

pub mod interface {
    pub mod sample {
        pub struct ISample {
            m_handle: u64,
        }

        impl ISample {
            pub const INTERFACE_ID: u64 = 9421138728380728032u64;
            pub const INTERFACE_CHECKSUM: u64 = 8375695449026144995u64;
            pub fn do_somthing_1(&self, i1: i32, i2: i32) -> i32 {
                crate::arieo::sample::i_sample::do_somthing1(self.m_handle, i1, i2)
            }

            pub fn do_somthing_2(&self, i1: i32, i2: i32) -> i32 {
                crate::arieo::sample::i_sample::do_somthing2(self.m_handle, i1, i2)
            }

        }

        impl From<u64> for ISample {
            fn from(handle: u64) -> Self {
                ISample { m_handle: handle }
            }
        }
    }
}
