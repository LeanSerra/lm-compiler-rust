cargo build --release
if [[ $? -eq 0 ]] then
    mv ./target/release/lm-compiler .
    ./lm-compiler inputs/test.txt
    dot -Tpng inputs/test.dot -o out.png

    rm lm-compiler
fi
