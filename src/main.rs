extern crate natex;

fn main() -> std::io::Result<()> {
    let o = natex::latex();
    let f = std::fs::File::open("a")?;
    let r = natex::reader(o, f);

    r.read()
}
