#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
#[non_exhaustive]
pub enum BootInfoTagType {
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
pub struct BootInfoHeader {
    pub total_size: u32,
    pub _reserved: u32,
}

#[repr(C)]
pub struct BootInfoTagHeader {
    pub tag_type: u32,
    pub size: u32,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
#[non_exhaustive]
pub enum MemoryMapType {
    RAM = 1,
    ACPI = 3,
    HibernationReserved = 4,
    BadMemory = 5,
}

#[repr(C)]
pub struct MemoryMapHeader {
    pub entry_size: u32,
    pub entry_version: u32,
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

#[repr(C)]
pub struct ModuleHeader {
    pub mod_start: u32,
    pub mod_end: u32,
}

// multiboot_tag_elf_sections definition in MB2 documentation is actually incorrect
#[repr(C)]
pub struct ElfSymbolInfoHeader {
    pub num: u32,
    pub entsize: u32,
    pub shndx: u32,
}

pub const MBI2_TAG_ALIGN: usize = 8;
