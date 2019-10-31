ifeq ($(shell uname),Darwin)
	EXT := dylib
else
	EXT := so
endif

# TODO: variable file name ending based on OS being used
# On MacOS, rename libstring_sum.dylib to string_sum.so, on Windows libstring_sum.dll to string_sum.pyd and on Linux libstring_sum.so to string_sum.so

all: target/release/my_library.dll
	move "target\release\my_library.dll" "src\my_library.pyd"
	python src/main.py

target/release/my_library.dll: src/lib.rs Cargo.toml
	cargo build --release

clean:
	cargo clean
