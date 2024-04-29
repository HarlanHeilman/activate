// Activate wrapper for python venv
// $ current_dir/Scripts/activate
// -> test/Scripts/activate

use std::env;
use std::process::Command;

fn main() {
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
    let module_name = current_dir.file_name().unwrap();
    print!("{}", module_name.to_str().unwrap());
    let python_venv = current_dir.join(module_name).join("Scripts").join("activate.bat");
    println!("{}", python_venv.display());
    let python_venv_clone = python_venv.clone(); // Clone the python_venv variable
    let output = Command::new(python_venv)
        .output()
        .expect(&format!("{} is not found", python_venv_clone.to_str().unwrap())); // Use the cloned variable in the expect method
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
