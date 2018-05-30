extern crate failure;
extern crate file_extender;

fn main() -> Result<(), failure::Error> {
    file_extender::run()
}
