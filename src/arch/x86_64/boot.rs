use crate::arch::{mbi::*, x86::elf32::SHT_STRTAB};
use crate::{arch::serial::COM1, util::str_from_cstr};
use core::{fmt::Write, slice::from_raw_parts, writeln};

use super::multiboot2::{BootInfoReader, BootInfoTag};

const KERNEL_BASE: usize = 0;

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
        asm!("lea {0}, [rip+0]", out(reg) rip);
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
            BootInfoTag::BootLoaderName(s) => writeln!(*COM1.lock(), "Bootloader: {}", s).unwrap(),
            BootInfoTag::BasicMeminfo(mi) => {
                writeln!(
                    *COM1.lock(),
                    "Basic meminfo: min {:#x}, max {:#x}",
                    mi.mem_lower,
                    mi.mem_upper
                )
                .unwrap();
            }
            BootInfoTag::Module(mi) => {
                writeln!(
                    *COM1.lock(),
                    "module from {:#x} to {:#x}: {}",
                    mi.mod_start,
                    mi.mod_end,
                    mi.string
                )
                .unwrap();
            }
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
            BootInfoTag::ElfSections(sih, sections) => {
                let strtab = sections.get(sih.shndx as usize).unwrap();
                assert_eq!(strtab.sh_type, SHT_STRTAB);
                let strbuf: &[u8] = unsafe {
                    from_raw_parts(
                        (KERNEL_BASE + strtab.sh_addr as usize) as *const u8,
                        strtab.sh_size as usize,
                    )
                };
                for sect in sections {
                    let sectname = str_from_cstr(strbuf, sect.sh_name as usize);
                    writeln!(*COM1.lock(), "Section {}", sectname.unwrap_or("")).unwrap();
                }
            }
            BootInfoTag::Unknown => {}
            _ => {}
        };
        writeln!(*COM1.lock()).unwrap();
    }

    crate::start();
}
