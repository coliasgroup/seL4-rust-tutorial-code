//
// Copyright 2024, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

use core::ptr;

use one_shot_mutex::sync::RawOneShotMutex;

use sel4_dlmalloc::{StaticDlmalloc, StaticHeap};
use sel4_panicking::catch_unwind;
use sel4_panicking_env::abort;

use crate::main;

const GRANULE_SIZE: usize = sel4::FrameObjectType::GRANULE.bytes(); // 4096

const STACK_SIZE: usize = 1024 * 16;

sel4_runtime_common::declare_stack!(STACK_SIZE);

const HEAP_SIZE: usize = 1024 * 64;

static STATIC_HEAP: StaticHeap<HEAP_SIZE> = StaticHeap::new();

#[global_allocator]
static GLOBAL_ALLOCATOR: StaticDlmalloc<RawOneShotMutex> =
    StaticDlmalloc::new(STATIC_HEAP.bounds());

sel4_panicking_env::register_debug_put_char!(sel4::debug_put_char);

sel4_runtime_common::declare_entrypoint! {
    () -> ! {
        unsafe {
            sel4::set_ipc_buffer(get_ipc_buffer().as_mut().unwrap());
        }

        match catch_unwind(main) {
            #[allow(unreachable_patterns)]
            Ok(never) => never,
            Err(_) => abort!("main() panicked"),
        }
    }
}

fn get_ipc_buffer() -> *mut sel4::IpcBuffer {
    addr_of_page_beyond_image(0) as *mut sel4::IpcBuffer
}

pub(crate) fn addr_of_page_beyond_image(index: usize) -> usize {
    extern "C" {
        static _end: usize;
    }
    (ptr::addr_of!(_end) as usize).next_multiple_of(GRANULE_SIZE) + index * GRANULE_SIZE
}
