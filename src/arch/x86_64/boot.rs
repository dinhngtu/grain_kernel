use crate::arch::multiboot2::*;
use crate::arch::serial::COM1;
use core::fmt::Write;

fn memory_type(num: u32) -> &'static str {
    match num {
        x if x == MemoryMapType::RAM as u32 => "RAM",
        x if x == MemoryMapType::ACPI as u32 => "ACPI",
        x if x == MemoryMapType::HibernationReserved as u32 => "HibernationReserved",
        x if x == MemoryMapType::BadMemory as u32 => "BadMemory",
        _ => "Reserved",
    }
}

fn print_rip(ptr: *const u8) {
    let mut rip: usize;
    unsafe {
        asm!("lea $0, [rip+0]" : "=r"(rip) ::: "intel","volatile");
    }
    writeln!(
        *COM1.lock(),
        "Hello from {:#x}, mbi={:#x}",
        rip,
        ptr as usize
    )
    .unwrap();
}

#[no_mangle]
pub extern "sysv64" fn x86_64_start(ptr: *const u8) -> ! {
    print_rip(ptr);
    for tag in BootInfoReader::from(ptr) {
        match tag {
            BootInfoTag::Cmdline(s) => writeln!(*COM1.lock(), "Command line: {}", s).unwrap(),
            BootInfoTag::Mmap(maps) => {
                for map in maps {
                    writeln!(
                        *COM1.lock(),
                        "Memory map: base_addr {:#x}, length {:#x}, map_type {} ({})",
                        map.base_addr,
                        map.length,
                        map.map_type,
                        memory_type(map.map_type)
                    )
                    .unwrap();
                }
            }
            BootInfoTag::Unknown => {}
            _ => {}
        };
    }

    crate::start();
}
