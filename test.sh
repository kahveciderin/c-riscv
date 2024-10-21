./test-gcc.sh $1
GCC_RET_VAL=$?
./test-toy.sh $1
TOY_RET_VAL=$?

if [ $GCC_RET_VAL -ne $TOY_RET_VAL ]; then
    echo "Return values differ: gcc: $GCC_RET_VAL, toy: $TOY_RET_VAL"
    exit 1
else
    echo "Return values are the same: $GCC_RET_VAL"
    exit 0
fi
