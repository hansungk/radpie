/nscratch/yrh/vortex-install/llvm-muon/bin/clang -Xclang -target-feature -Xclang +vortex -mllvm -vortex-branch-divergence=0 --target=riscv32-unknown-elf -mabi=ilp32f  -march=rv32imaf --sysroot=$RISCV_TOOLCHAIN_PATH/riscv32-unknown-elf --gcc-toolchain=$RISCV_TOOLCHAIN_PATH -fuse-ld=lld -v -fno-rtti -fno-exceptions -nostartfiles -fdata-sections -ffunction-sections  -Wl,-v,-Bstatic,-T,/scratch/yrh/vxsw/kernel/linker/vx_link32.ld,--defsym=STARTUP_ADDR=0x80000000,-z,norelro -O2 vecadd.c -o vecadd.elf
/nscratch/yrh/vortex-install/llvm-muon/bin/clang -Xclang -target-feature -Xclang +vortex -mllvm -vortex-branch-divergence=0 --target=riscv32-unknown-elf -mabi=ilp32f  -march=rv32imaf --sysroot=$RISCV_TOOLCHAIN_PATH/riscv32-unknown-elf --gcc-toolchain=$RISCV_TOOLCHAIN_PATH -fuse-ld=lld -v -fno-rtti -fno-exceptions -nostartfiles -fdata-sections -ffunction-sections  -Wl,-v,-Bstatic,-T,/scratch/yrh/vxsw/kernel/linker/vx_link32.ld,--defsym=STARTUP_ADDR=0x80000000,-z,norelro -O2 vecadd.c -S -o vecadd.S
/nscratch/yrh/vortex-install/llvm-muon/bin/llvm-objdump -d vecadd.elf > vecadd.dump

/nscratch/yrh/vortex-install/llvm-muon-nobank/bin/clang -Xclang -target-feature -Xclang +vortex -mllvm -vortex-branch-divergence=0 --target=riscv32-unknown-elf -mabi=ilp32f  -march=rv32imaf --sysroot=$RISCV_TOOLCHAIN_PATH/riscv32-unknown-elf --gcc-toolchain=$RISCV_TOOLCHAIN_PATH -fuse-ld=lld -v -fno-rtti -fno-exceptions -nostartfiles -fdata-sections -ffunction-sections  -Wl,-v,-Bstatic,-T,/scratch/yrh/vxsw/kernel/linker/vx_link32.ld,--defsym=STARTUP_ADDR=0x80000000,-z,norelro -O2 vecadd.c -o vecadd.nobank.elf
/nscratch/yrh/vortex-install/llvm-muon-nobank/bin/clang -Xclang -target-feature -Xclang +vortex -mllvm -vortex-branch-divergence=0 --target=riscv32-unknown-elf -mabi=ilp32f  -march=rv32imaf --sysroot=$RISCV_TOOLCHAIN_PATH/riscv32-unknown-elf --gcc-toolchain=$RISCV_TOOLCHAIN_PATH -fuse-ld=lld -v -fno-rtti -fno-exceptions -nostartfiles -fdata-sections -ffunction-sections  -Wl,-v,-Bstatic,-T,/scratch/yrh/vxsw/kernel/linker/vx_link32.ld,--defsym=STARTUP_ADDR=0x80000000,-z,norelro -O2 vecadd.c -S -o vecadd.nobank.S
/nscratch/yrh/vortex-install/llvm-muon-nobank/bin/llvm-objdump -d vecadd.nobank.elf > vecadd.nobank.dump

/nscratch/yrh/vortex-install/llvm-muon-baseline/bin/clang -Xclang -target-feature -Xclang +vortex -mllvm -vortex-branch-divergence=0 --target=riscv32-unknown-elf -mabi=ilp32f  -march=rv32imaf --sysroot=$RISCV_TOOLCHAIN_PATH/riscv32-unknown-elf --gcc-toolchain=$RISCV_TOOLCHAIN_PATH -fuse-ld=lld -v -fno-rtti -fno-exceptions -nostartfiles -fdata-sections -ffunction-sections  -Wl,-v,-Bstatic,-T,/scratch/yrh/vxsw/kernel/linker/vx_link32.ld,--defsym=STARTUP_ADDR=0x80000000,-z,norelro -O2 vecadd.c -o vecadd.baseline.elf
/nscratch/yrh/vortex-install/llvm-muon-baseline/bin/clang -Xclang -target-feature -Xclang +vortex -mllvm -vortex-branch-divergence=0 --target=riscv32-unknown-elf -mabi=ilp32f  -march=rv32imaf --sysroot=$RISCV_TOOLCHAIN_PATH/riscv32-unknown-elf --gcc-toolchain=$RISCV_TOOLCHAIN_PATH -fuse-ld=lld -v -fno-rtti -fno-exceptions -nostartfiles -fdata-sections -ffunction-sections  -Wl,-v,-Bstatic,-T,/scratch/yrh/vxsw/kernel/linker/vx_link32.ld,--defsym=STARTUP_ADDR=0x80000000,-z,norelro -O2 vecadd.c -S -o vecadd.baseline.S
/nscratch/yrh/vortex-install/llvm-muon-baseline/bin/llvm-objdump -d vecadd.baseline.elf > vecadd.baseline.dump
