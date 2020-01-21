use crate::arch::serial::COM1;
use core::convert::{From, TryInto};
use core::ffi::c_void;
use core::fmt::Write;
use core::mem::size_of;

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
#[allow(dead_code)]
#[non_exhaustive]
enum BootInfoTagType {
    End = 0,
    Cmdline,
    BootLoaderName,
    Module,
    BasicMeminfo,
    Bootdev,
    Mmap,
    Vbe,
    Framebuffer,
    ElfSections,
    Apm,
    Efi32,
    Efi64,
    Smbios,
    AcpiOld,
    AcpiNew,
    Network,
    EfiMmap,
    EfiBs,
    Efi32Ih,
    Efi64Ih,
    LoadBaseAddr,
}

#[repr(C)]
struct BootInfoHeader {
    total_size: u32,
    _reserved: u32,
}

#[repr(C)]
struct BootInfoTagBase {
    tag_type: u32,
    size: u32,
}

pub struct BootInfoReader<'a> {
    buffer: &'a [u8],
    offset: usize,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
#[allow(dead_code)]
#[non_exhaustive]
pub enum MemoryMapType {
    RAM = 1,
    ACPI = 3,
    HibernationReserved = 4,
    BadMemory = 5,
}

#[repr(C)]
pub struct MemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub map_type: u32,
    pub reserved: u32,
}

#[allow(dead_code)]
pub enum BootInfoTag<'a> {
    Cmdline(&'a str),
    BootLoaderName,
    Module,
    BasicMeminfo,
    Bootdev,
    Mmap(&'a [MemoryMapEntry]),
    Vbe,
    Framebuffer,
    ElfSections,
    Apm,
    Efi32,
    Efi64,
    Smbios,
    AcpiOld,
    AcpiNew,
    Network,
    EfiMmap,
    EfiBs,
    Efi32Ih,
    Efi64Ih,
    LoadBaseAddr,
    Unknown,
}

const MBI2_TAG_ALIGN: usize = 8;
const MBI2_HEADER_LEN: usize = size_of::<BootInfoHeader>();
const MBI2_TAG_BASE_LEN: usize = size_of::<BootInfoTagBase>();

impl<'a> From<*const c_void> for BootInfoReader<'a> {
    fn from(ptr: *const c_void) -> BootInfoReader<'a> {
        let header = unsafe { ptr.cast::<BootInfoHeader>().as_ref() }.unwrap();
        if (header.total_size as usize) < MBI2_HEADER_LEN {
            panic!("invalid mbi2 header size");
        }
        writeln!(
            *COM1.lock(),
            "header total_size {} reserved {}",
            header.total_size,
            header._reserved
        )
        .unwrap();
        let total_buffer =
            unsafe { core::slice::from_raw_parts(ptr.cast::<u8>(), header.total_size as usize) };
        BootInfoReader {
            buffer: total_buffer.get(MBI2_HEADER_LEN..).unwrap(),
            offset: 0,
        }
    }
}

fn from_ne_bytes(buf: &[u8]) -> Option<u32> {
    if buf.len() != 4 {
        return None;
    }
    match TryInto::<[u8; 4]>::try_into(buf) {
        Ok(bytes) => Some(u32::from_ne_bytes(bytes)),
        _ => None,
    }
}

impl<'a> Iterator for BootInfoReader<'a> {
    type Item = BootInfoTag<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        {
            writeln!(*COM1.lock(), "offset {}", self.offset).unwrap();
        }
        if self.offset < self.buffer.len() {
            // BootInfoTagBase *tag_base = self.buffer + offset
            let tag_base: &BootInfoTagBase = self
                .buffer
                .get(self.offset..self.offset + MBI2_TAG_BASE_LEN)
                .map(|x| x.as_ptr())
                .map(|x| x.cast::<BootInfoTagBase>())
                .and_then(|x| unsafe { x.as_ref() })
                .unwrap();
            {
                writeln!(
                    *COM1.lock(),
                    "tag type {} size {}",
                    tag_base.tag_type as u32,
                    tag_base.size
                )
                .unwrap();
            }
            // the part after the standard tag header (type + size)
            let tag_data = self
                .buffer
                .get((self.offset + MBI2_TAG_BASE_LEN)..(self.offset + tag_base.size as usize))
                .unwrap();
            let tag = match tag_base.tag_type {
                x if x == BootInfoTagType::Cmdline as u32 => {
                    let cmdline = core::str::from_utf8(tag_data).unwrap();
                    Some(BootInfoTag::Cmdline(cmdline))
                }
                x if x == BootInfoTagType::Mmap as u32 => {
                    let entry_size: u32 = tag_data.get(0..4).and_then(from_ne_bytes).unwrap();
                    let entry_version: u32 = tag_data.get(4..8).and_then(from_ne_bytes).unwrap();
                    writeln!(
                        *COM1.lock(),
                        "memory map size {} version {}",
                        entry_size,
                        entry_version
                    )
                    .unwrap();
                    if entry_size != 24 || entry_version != 0 {
                        panic!("unsupported memory map version");
                    }
                    if (tag_data.len() - 8) % (entry_size as usize) != 0 {
                        panic!("invalid memory map count");
                    }
                    let maps = unsafe {
                        core::slice::from_raw_parts(
                            tag_data.get(8..)?.as_ptr().cast::<MemoryMapEntry>(),
                            (tag_data.len() - 8) / (entry_size as usize),
                        )
                    };
                    Some(BootInfoTag::Mmap(maps))
                }
                x if x == BootInfoTagType::End as u32 => None,
                _ => Some(BootInfoTag::Unknown),
            };
            self.offset += tag_base.size as usize;
            self.offset += (MBI2_TAG_ALIGN - self.offset % MBI2_TAG_ALIGN) % MBI2_TAG_ALIGN;
            tag
        } else {
            panic!("reached end of mbi2");
        }
    }
}
