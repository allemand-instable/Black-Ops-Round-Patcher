use std::fs;
use std::path::Path;
use clearscreen::clear;
// use std::io::stdin;

// https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}



fn main() {
    loop {
        
        println!("Please enter a round patch \n - 1 for original patch file \n - \"quit\" to exit program \n your input : ");
        read!(input_round as String);
        
        if input_round == "quit"
        {
            break;
        }
        else 
        {
            copy_patch_to_zone_folder( input_round);
        }


    }
}

fn copy_patch_to_zone_folder(round_number : String) -> std::io::Result<()>
{
    let root_path = "./Patches/";

    let defaut_path_backup = "/common_zombie_patch.ff";

    let round_patch_path;

    if round_number == "1" {    
        round_patch_path = ["bkp/common_zombie_patch.ff"].join("");
    }

    else {
        round_patch_path = ["R", &round_number,  defaut_path_backup].join("");
    }

    
    let from_path = [root_path, &round_patch_path].join("");
    if path_exists(&from_path)
    {
        fs::copy(from_path, "./zone/Common/common_zombie_patch.ff")?;
        clear();
        println!("Copied !");
    }
    else 
    {
        clear();
        println!("The folder does not exist, try another round number...\n");
    }
    return Ok(())

}


fn path_exists( path_String_ref : &String ) -> bool
{
    let path_exists_bool : bool = Path::new(path_String_ref).exists();
    println!("{} exists ? {}", path_String_ref, path_exists_bool);
    return path_exists_bool
}