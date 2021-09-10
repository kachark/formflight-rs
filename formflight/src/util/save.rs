
use std::io;
use std::error::Error;
use serde::Serialize;

pub fn to_csv<T: Serialize>(data: &T) -> Result<(), Box<dyn Error>> {

    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.serialize(data)?;
    wtr.flush()?;

    Ok(())

}
