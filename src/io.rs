use std::fs;

/// Save an array of `u8`s in a file
pub fn write_bytes(bytes: &[u8], fname: &str) -> std::io::Result<()> 
{
    fs::write(fname, bytes)
}

/// Read a file as a `Vec<u8>`
pub fn read_bytes(fname: &str) -> std::io::Result<Vec<u8>>
{
    fs::read(fname)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn write_read_1() {

        let fname = "test_io_1.dat";

        // string to save
        let phrase = "this is a test";

        // create the temp directory
        fs::create_dir_all("temp").unwrap();

        // save the string as bytes
        write_bytes(phrase.as_bytes(), &("temp/".to_owned()+fname)).unwrap();

        // read the file content
        let bytes = read_bytes(&("temp/".to_owned()+fname)).unwrap();

        assert_eq!(*phrase.as_bytes(), bytes);
    }
}
