#[repr(C)]
pub struct ElfSectionHeader {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

/* sh_type */
pub const SHT_NULL: u32 = 0;
pub const SHT_PROGBITS: u32 = 1;
pub const SHT_SYMTAB: u32 = 2;
pub const SHT_STRTAB: u32 = 3;
pub const SHT_RELA: u32 = 4;
pub const SHT_HASH: u32 = 5;
pub const SHT_DYNAMIC: u32 = 6;
pub const SHT_NOTE: u32 = 7;
pub const SHT_NOBITS: u32 = 8;
pub const SHT_REL: u32 = 9;
pub const SHT_SHLIB: u32 = 10;
pub const SHT_DYNSYM: u32 = 11;
pub const SHT_INIT_ARRAY: u32 = 14;
pub const SHT_FINI_ARRAY: u32 = 15;
pub const SHT_PREINIT_ARRAY: u32 = 16;
pub const SHT_GROUP: u32 = 17;
pub const SHT_SYMTAB_SHNDX: u32 = 18;
pub const SHT_NUM: u32 = 19;

/* sh_flags */
pub const SHF_WRITE: u32 = 1 << 0;
pub const SHF_ALLOC: u32 = 1 << 1;
pub const SHF_EXECINSTR: u32 = 1 << 2;
pub const SHF_MERGE: u32 = 1 << 4;
pub const SHF_STRINGS: u32 = 1 << 5;
pub const SHF_INFO_LINK: u32 = 1 << 6;
pub const SHF_LINK_ORDER: u32 = 1 << 7;
pub const SHF_OS_NONCONFORMING: u32 = 1 << 8;
pub const SHF_GROUP: u32 = 1 << 9;
pub const SHF_TLS: u32 = 1 << 10;
pub const SHF_COMPRESSED: u32 = 1 << 11;

/* special section indexes */
pub const SHN_UNDEF: u32 = 0;
pub const SHN_LORESERVE: u32 = 0xff00;
pub const SHN_LOPROC: u32 = 0xff00;
pub const SHN_HIPROC: u32 = 0xff1f;
pub const SHN_LOOS: u32 = 0xff20;
pub const SHN_HIOS: u32 = 0xff3f;
pub const SHN_ABS: u32 = 0xfff1;
pub const SHN_COMMON: u32 = 0xfff2;
pub const SHN_XINDEX: u32 = 0xffff;
pub const SHN_HIRESERVE: u32 = 0xffff;
