//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use microkit_shared_memory_common::{RegionB, REGION_A_SIZE};

use sel4_microkit::{
    debug_println, memory_region_symbol, protection_domain, Channel, Handler, MessageInfo,
    NullHandler,
};
use sel4_shared_memory::{map_field, SharedMemoryRef};

const SERVER: Channel = Channel::new(13);

#[protection_domain]
fn init() -> impl Handler {
    debug_println!("client: initializing");

    let mut region_a = unsafe {
        SharedMemoryRef::new(memory_region_symbol!(region_a_vaddr: *mut [u8], n = REGION_A_SIZE))
    };

    let mut region_b =
        unsafe { SharedMemoryRef::new(memory_region_symbol!(region_b_vaddr: *mut RegionB)) };

    debug_println!("client: region_a = {region_a:#x?}");
    debug_println!("client: region_b = {region_b:#x?}");

    region_a.as_mut_ptr().index(13).write(37);

    let region_b_ptr = region_b.as_mut_ptr();
    map_field!(region_b_ptr.foo).as_slice().index(1).write(23);

    let _ = SERVER.pp_call(MessageInfo::default());

    debug_println!("TEST_PASS");

    NullHandler::new()
}
