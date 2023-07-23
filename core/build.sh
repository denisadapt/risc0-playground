/opt/riscv/bin/riscv32-unknown-elf-g++ -march=rv32im -mabi=ilp32 -mcmodel=medany -static -nostartfiles -nostdlib -fno-exceptions mylib.cpp --sysroot=/opt/riscv/riscv32-unknown-elf -c
/opt/riscv/bin/riscv32-unknown-elf-ar rsc libmylib.a mylib.o

/opt/riscv/bin/riscv32-unknown-elf-g++ -march=rv32im -mabi=ilp32 -mcmodel=medany -static -nostartfiles -nostdlib -fno-exceptions main.cpp --sysroot=/opt/riscv/riscv32-unknown-elf -c

/opt/riscv/bin/riscv32-unknown-elf-ld main.o libmylib.a /opt/riscv/riscv32-unknown-elf/lib/libc.a -relocatable -o superlib.o

cp superlib.o ../methods/guest/src

rm *.o
rm *.a

