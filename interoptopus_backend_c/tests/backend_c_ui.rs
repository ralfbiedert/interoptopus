use interoptopus::writer::IndentWriter;
use interoptopus::Error;
use std::fs::{read_to_string, File};

#[test]
fn generated_matches_expected() -> Result<(), Error> {
    use interoptopus_backend_c::{Config, Generator, Interop};

    let library = interoptopus_reference_project::ffi_inventory();

    let config = Config { ..Config::default() };

    let generator = Generator::new(config, library);

    {
        let mut file = File::create("tests/output/my_header.h.generated")?;
        let mut writer = IndentWriter::new(&mut file, "    ");

        generator.write_to(&mut writer)?;
    }

    let actual = read_to_string("tests/output/my_header.h.generated")?;
    let expected = read_to_string("tests/output/my_header.h")?;

    assert_eq!(expected, actual);

    Ok(())
}
