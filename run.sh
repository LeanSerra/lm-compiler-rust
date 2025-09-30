cargo build --release
if [[ $? -eq 0 ]] then
    mv ./target/release/lm-compiler .
    ./lm-compiler inputs/test.txt
    rm lm-compiler
fi
