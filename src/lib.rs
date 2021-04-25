use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use mmap::MemoryMap;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("jit_compile", op_compile);
}

fn op_compile(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let code = zero_copy[0].to_vec();
    let arguments = zero_copy[1].to_vec();
    {
        let rwx = &[
            mmap::MapOption::MapReadable,
            mmap::MapOption::MapWritable,
            mmap::MapOption::MapExecutable,
        ];

        let mapping = MemoryMap::new(code.len(), rwx).unwrap();
        unsafe {
            std::ptr::copy(code.as_ptr(), mapping.data(), code.len());
        }

        let func: fn(_: u8, _: u8, _: u8, _: u8) -> u8 =
            unsafe { std::mem::transmute(mapping.data()) };
        let result = match arguments.len() {
            0 => func(0, 0, 0, 0),
            1 => func(arguments[0], 0, 0, 0),
            2 => func(arguments[0], arguments[1], 0, 0),
            3 => func(arguments[0], arguments[1], arguments[2], 0),
            4 => func(arguments[0], arguments[1], arguments[2], arguments[3]),
            _ => unreachable!(),
        };

        let result_box: Box<[u8]> = vec![result].into_boxed_slice();
        Op::Sync(result_box)
    }
}
