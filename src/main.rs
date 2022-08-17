mod blrs;
use blrs::parse_args;

fn main() {
    if let Err(e) = parse_args() {
        e.exec();
    };
}
