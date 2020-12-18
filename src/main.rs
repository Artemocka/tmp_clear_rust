use std::path::Path;

fn clear_tmp_dir() -> std::io::Result<()> {
    let temp_dir = std::env::temp_dir();
    
    for elem in temp_dir.read_dir()? {
       

        let entry = elem?;
        if entry.path().is_dir(){
            std::fs::remove_dir_all(entry.path())?;

        } else {
            std::fs::remove_file(entry.path())?;
        }
        
    }
    Ok(())
}

fn main() {
    clear_tmp_dir();
}
