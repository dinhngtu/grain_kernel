QEMU ?= qemu-system-x86_64

run-wsl: QEMU="/mnt/c/Program Files/qemu/qemu-system-x86_64.exe"
run-wsl: QEMU_FLAGS+=-L "C:\\Program Files\\qemu"

$(kernel): $(kernel_dbg)
	objcopy --strip-unneeded -I elf64-x86-64 -O elf32-i386 $(kernel_dbg) $(kernel)
