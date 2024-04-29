use walkdir;

fn number_of_temp_files() -> Result<i32, &'static str>{
    let key = "TEMP";
    let temp_path;
    match std::env::var(key){
        Ok(val)=>{
            temp_path = String::from(val);
        }, Err(_e)=> {
            return Err("No environmental var called TEMP")
        }
    }
    println!("temp file path: {}", temp_path);

    fn count_files(pathname: String) -> i32{
        let mut path;
        let mut tmp;
        let mut flag = false;
        let mut res = 0;
        for entry in walkdir::WalkDir::new(pathname){
            tmp = entry.unwrap();
            path = tmp.path();
            if path.is_dir(){
                if flag == false {
                    flag = true;
                }
                else{
                    res += count_files(path.display().to_string());
                }
            }
            else {
                println!("file: {}", path.display());
                res += 1;
            }
        }
        return res;
    }

    return Ok(count_files(temp_path))
}

fn main() {
    match number_of_temp_files() {
       Ok(result) => println!("Number of temp files: {}", result),
       Err(result) => println!("Error: {}", result)
    }
}