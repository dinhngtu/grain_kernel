use crate::arch::serial::COM1;
use crate::util::*;
use core::convert::From;
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
struct BootInfoTagHeader {
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
struct MemoryMapHeader {
    entry_size: u32,
    entry_version: u32,
}

#[repr(C)]
pub struct MemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub map_type: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct BasicMeminfoHeader {
    pub mem_lower: u32,
    pub mem_upper: u32,
}

#[allow(dead_code)]
pub enum BootInfoTag<'a> {
    Cmdline(&'a str),
    BootLoaderName(&'a str),
    Module,
    BasicMeminfo(&'a BasicMeminfoHeader),
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

impl<'a> From<*const u8> for BootInfoReader<'a> {
    fn from(ptr: *const u8) -> BootInfoReader<'a> {
        let header = unsafe { ptr.cast::<BootInfoHeader>().as_ref() }.unwrap();
        assert!((header.total_size as usize) >= size_of::<BootInfoHeader>());
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
            buffer: total_buffer.get(size_of::<BootInfoHeader>()..).unwrap(),
            offset: 0,
        }
    }
}

impl<'a> Iterator for BootInfoReader<'a> {
    type Item = BootInfoTag<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        writeln!(*COM1.lock(), "offset {}", self.offset).unwrap();

        let tag_begin = self.buffer.split_at(self.offset).1;
        // offset from after tag header
        let tag_offset = size_of::<BootInfoTagHeader>();
        let tag_header: &BootInfoTagHeader = tag_begin
            .get(..tag_offset)
            .and_then(|x| unsafe { from_bytes(x) })
            .unwrap();
        let tag_size = tag_header.size as usize;
        writeln!(
            *COM1.lock(),
            "tag type {} size {}",
            tag_header.tag_type,
            tag_size
        )
        .unwrap();

        let tag_data = tag_begin.get(tag_offset..tag_size).unwrap();
        let tag = match tag_header.tag_type {
            x if x == BootInfoTagType::Cmdline as u32 => {
                let cmdline = core::str::from_utf8(tag_data).unwrap();
                Some(BootInfoTag::Cmdline(cmdline))
            }
            x if x == BootInfoTagType::BootLoaderName as u32 => {
                let blname = core::str::from_utf8(tag_data).unwrap();
                Some(BootInfoTag::BootLoaderName(blname))
            }
            x if x == BootInfoTagType::BasicMeminfo as u32 => {
                let mi: &BasicMeminfoHeader = unsafe { from_bytes(tag_data).unwrap() };
                Some(BootInfoTag::BasicMeminfo(mi))
            }
            x if x == BootInfoTagType::Mmap as u32 => {
                let (mmap_head, maps_raw) = tag_data.split_at(8);
                let mmap_base: &MemoryMapHeader = unsafe { from_bytes(mmap_head).unwrap() };
                writeln!(
                    *COM1.lock(),
                    "memory map size {} version {}",
                    mmap_base.entry_size,
                    mmap_base.entry_version
                )
                .unwrap();
                assert_eq!(mmap_base.entry_size, 24);
                assert_eq!(mmap_base.entry_version, 0);
                assert_eq!(maps_raw.len() % (mmap_base.entry_size as usize), 0);
                let maps: &[MemoryMapEntry] = unsafe { slice_from_bytes(maps_raw).unwrap() };
                Some(BootInfoTag::Mmap(maps))
            }
            x if x == BootInfoTagType::End as u32 => None,
            _ => Some(BootInfoTag::Unknown),
        };

        self.offset += tag_size;
        self.offset += (MBI2_TAG_ALIGN - self.offset % MBI2_TAG_ALIGN) % MBI2_TAG_ALIGN;
        tag
    }
}
