echo "Running file $1 with spike"

riscv64-unknown-elf-gcc output/$1.s -o output/$1.out -march=rv32imafdc -mabi=ilp32

if [ $? -ne 0 ]; then
    echo "Error compiling file $1"
    exit 1
fi

spike --isa=RV32IMAFDC pk output/$1.out
RET_VAL=$?
echo "Return value: $RET_VAL"
exit $RET_VAL
