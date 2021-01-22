arch ?= x86_64
kernel_dbg := build/kernel-$(arch).dbg
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/kernel.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/asm/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/asm/%.asm, \
    build/arch/$(arch)/%.o, $(assembly_source_files))

target ?= $(arch)-grain
rust_os := target/$(target)/debug/libgrain_kernel.a

ifeq ($(DEBUG), 1)
	QEMU_FLAGS += -S -gdb tcp::9000
endif

.PHONY: all clean run run-wsl iso kernel

all: $(iso)

include src/arch/$(arch)/Makefile.inc

clean:
	rm -rf build
	rm -rf target

run: $(iso)
	$(QEMU) -cdrom $(iso) -nographic $(QEMU_FLAGS)

run-wsl: $(iso)
	$(QEMU) -machine q35,accel=whpx -cdrom $(iso) -nographic $(QEMU_FLAGS)

$(kernel_dbg): cargo $(rust_os) $(assembly_object_files) $(linker_script)
	ld --gc-sections -T $(linker_script) -o $(kernel_dbg) $(assembly_object_files) $(rust_os)

kernel: $(kernel)

iso: $(iso)

$(iso): $(kernel)
	rm -f $(iso)
	mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/
	cp $(grub_cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles

cargo:
	cargo xbuild --target ./$(target).json

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/asm/%.asm
	mkdir -p $(shell dirname $@)
	nasm -g -felf64 $< -o $@
