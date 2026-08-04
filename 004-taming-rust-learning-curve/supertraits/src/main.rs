use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;

trait Saveable: Display {
    fn save<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut file = File::create(path.as_ref())?;
        writeln!(file, "{}", self.to_string())?;

        Ok(())
    }
}
fn main() {
    println!("Hello, world!");
}
