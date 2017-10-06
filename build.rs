
extern crate tango;

fn main() {

    let mut config = tango::Config::new();
    config.emit_rerun_if();
    tango::process_root_with_config(config).unwrap();

}
