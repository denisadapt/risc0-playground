fn main() {
    cc::Build::new().object("src/superlib.o").compile("superlib");
    cc::Build::new().object("src/myblake2b.o").compile("myblake2b");
}
