echo "Compiling file $1 with gcc"

riscv64-unknown-elf-gcc tests/$1.c -o output/$1-gcc.out -march=rv32imafdc -mabi=ilp32

if [ $? -ne 0 ]; then
    echo "Error compiling file $1"
    exit 1
fi

spike --isa=RV32IMAFDC pk output/$1-gcc.out
RET_VAL=$?
echo "Return value: $RET_VAL"
exit $RET_VAL
