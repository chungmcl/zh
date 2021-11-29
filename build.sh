if [ $1 = "rust" ]; then
    python3 generate_header.py rust && rustc -C prefer-dynamic rust/jyutping.rs 
fi
if [ $1 = "c" ]; then
    python3 generate_header.py c && gcc -o jyutping c/jyutping.c
fi
exit 1