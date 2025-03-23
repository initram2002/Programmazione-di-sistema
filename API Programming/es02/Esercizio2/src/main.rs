use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // input string
    slug_in: String,
}

fn slugify(s: &str) -> String {

}

fn conv(c: char) -> char {
    const SUBS_I : &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóoeøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O : &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
}

fn main() {
    let args = Args::parse();
}
