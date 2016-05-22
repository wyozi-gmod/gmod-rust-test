extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true) // Switch to C++ library compilation.
        .file("gmod-cpp/gmod-iface.cpp")
        .compile("libgmodiface.a");
}