
use crate::menue;
use crate::helpers;

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{stdout, Write};
use crossterm::style::*;
use std::process::Command;
use curl::easy::Easy;
use std::fmt::Display;

pub fn set_up(){
    
    menue::print_title("DISTRIBUTION");
    menue::print_menue(&[
        menue::build_menue_point("calc", "calculate Atlas of Game-elements"),
        menue::build_menue_point("chek", "check if Project is pushable"), 
        menue::build_menue_point("push", "adds,commits and pushes all repos") 
    ]);

    check_input()
}

fn check_input(){
    match helpers::input_validator::sanitice().as_str() {
        "calc" => calculate(),
        "chek" => check(),
        "push" => push(),
         _     => print!("still WIP"),
    }
}





pub fn calculate(){
    menue::print_chapter("Set Back-up");
    menue::print_chapter("building Filepaths...");
    menue::print_chapter("Starts PatnicRoom...");


    let cached_gameinit_content = fs::read_to_string( get_game_init() ).expect("file not found");
    
    fs::remove_file( get_game_init() );
    fs::write(       get_game_init()  , "statistics");

    env::set_current_dir(helpers::filepaths::get_godot_exe()).is_ok();
    
    helpers::bash_commands::start_patnic_with_calc();
    helpers::text_formater::print_white("");


    fs::write( &get_game_init(), cached_gameinit_content );
    
    menue::print_chapter("Distributes Markdowns...");
}

fn get_game_init()->String{   return helpers::filepaths::get_game_init()   }







fn check(){
    let mut first_cond = _chek_changelog("https://raw.githubusercontent.com/Daniel-RRR/Patnic-Src/main/ChangeLog.md");
    



}



fn _chek_changelog(workspace:&str) -> bool{
    let mut data = String::new();
    let mut easy = Easy::new();
    
    easy.url(workspace).unwrap();

    let mut html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            html = String::from_utf8(Vec::from(data)).unwrap();        
            Ok(data.len())
        }).unwrap(); 

        transfer.perform().unwrap();
    }
    html.retain(|c| !c.is_whitespace());
    println!("{:?}\n\n", html);



    

    let mut contents = fs::read_to_string(format!("{}Patnic-Src/ChangeLog.md",helpers::filepaths::get_root()))
        .expect("Something went wrong reading the file");
    //let saniticed_content = contents.retain(|c| !c.is_whitespace());
    contents.retain(|c| !c.is_whitespace());
    println!("With text:\n{}",contents);
    

    if contents == html{
        println!("Same"); 
    }else{
        println!("not same"); 
    }
    return contents == html
}




fn push(){
    let mut contents = fs::read_to_string(format!("{}Patnic-Src/ChangeLog.md",helpers::filepaths::get_root()))
        .expect("Something went wrong reading the file");

    let mut isFirstEntryInFile = true;
    let mut commit_message : String;
    commit_message= "".to_owned();
        
    for line in contents.split("\n") {
        if line.contains("# Version") && !isFirstEntryInFile{ break }
        if line.contains("# Version") && isFirstEntryInFile{ isFirstEntryInFile = false; }
        if !isFirstEntryInFile{
            commit_message = format!("{}\n{}", commit_message, line);
        }
    }
    print!("{}",commit_message);
    helpers::bash_commands::push_repo(format!("{}/Patnic-Src",helpers::filepaths::get_root()).as_str(),commit_message.as_str())



}


