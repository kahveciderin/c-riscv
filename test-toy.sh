echo "Compiling file $1 with toy compiler"

cargo r $1

if [ $? -ne 0 ]; then
    echo "Error compiling file $1"
    exit 1
fi

echo "Running file $1 with spike"

./run-asm.sh $1