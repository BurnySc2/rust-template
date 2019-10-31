from ctypes import cdll
from sys import platform
import time

if platform == "darwin":
    prefix = "lib"
    ext = "dylib"
elif platform == "win32":
    prefix = ""
    ext = "dll"
else:
    prefix = "lib"
    ext = "so"


def my_factorial(n):
    if n == 1:
        return 1
    return n * my_factorial(n - 1)


t0 = time.perf_counter()
lib = cdll.LoadLibrary("target/debug/{}my_library.{}".format(prefix, ext))
factorial = lib.factorial

input = 12
t1 = time.perf_counter()
output = factorial(input)
t2 = time.perf_counter()

output2 = my_factorial(input)
t3 = time.perf_counter()


print(f"{input}! = Rust: {output} = Py: {output2}")
print(f"Time passed: {t1-t0} and {t2-t1} and {t3-t2}")
