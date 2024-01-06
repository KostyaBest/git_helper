
use std::process::{Command,Stdio,Output};
//use std::io::{self,Write};
use std::collections::{
    HashMap,
};


use std::env::{
   args_os,
};


pub fn git_pull(_args:Vec<String>)->Result<Option<Output>,CliError>
{
    match Command::new("git")
                .args(["pull"])
                .stdout(Stdio::piped())
                .output()
                .expect("Expect git pulling repo")
        {
            output => Ok(Some(output)),
            
        }

}


pub fn git_init(_args:Vec<String>)->Result<Option<Output>,CliError>
{
    //TODO add pathing here later
    let git_init_status = Command::new("git")
                        .args(["init"])
                        .status()
                        .expect("Expect creating git repo");

    match git_init_status.success()
    {
        true => Ok(None),
        _ => Err(CliError::CliCommandError),
    }

}

pub fn git_bare(_args:Vec<String>)->Result<Option<Output>,CliError>
{
    //TODO add pathing here later
    let git_bare_status = Command::new("git")
                        .args(["init","--bare"])
                        .status()
                        .expect("Expect git repo");

    match git_bare_status.success()
    {
        true => Ok(None),
        _ => Err(CliError::CliCommandError),
    }

}


pub fn git_commit(_args:Vec<String>)->Result<Option<Output>,CliError>
{
    if _args.len() < 1{
        return Err(CliError::NotEnoughArguments); 
    }
    let message = &_args[0];
    let command =format!("commit -m \"{message}\"");
    let git_commit = Command::new("git")
                        .args(["add .",&command])
                        .status()
                        .expect("Git push");
    match git_commit.success()
    {
        true => Ok(None),
        _ => Err(CliError::CliCommandError),
    }

}

pub fn help(_args:Vec<String>)->Result<Option<Output>,CliError>
{
    println!("#################HELP##################");
    println!("pull - git pulls");     
    println!("p - git pulls");     
    println!("init - git init");     
    println!("i - git init");     
    println!("commit - git commits");     
    println!("bare - git bare");     
    println!("help - print this help");     
    println!("#######################################");
    Ok(None)
}


pub enum CliError {
    CliCommandError,
    NotValidCommand,
    NotEnoughArguments

}

type CliCommand=fn(args:Vec<String>)->Result<Option<Output>,CliError>;

fn get_commands()->HashMap<String,CliCommand>{
    let mut commands=HashMap::<String,CliCommand>::new();
    
    commands.insert(String::from("pull"),git_pull);
    commands.insert(String::from("p"),git_pull);
    commands.insert(String::from("init"),git_init);
    commands.insert(String::from("i"),git_init);
    commands.insert(String::from("commit"),git_commit);
    commands.insert(String::from("bare"),git_bare);
    commands.insert(String::from("help"),help);
    commands
}

fn main() {
    let commands = get_commands();
    let mut _args:Vec<String> = vec![];

    if args_os().len() < 2 {
        commands["help"](vec![]); 
        return;
    }
    for arg in args_os(){

        _args.push(arg.into_string().unwrap());
    }
    
    if commands.contains_key(&_args[1])
    {
        //let result = commands[&_args[0]](&_args.as_slice(1,&_args.len()));
        //let result = commands[&_args[0]]((&_args.as_slice(1 as i32,&_args.len() as i32)).to_vec());
        //let result = commands[&_args[1]](_args);
        let result = commands[&_args[1]](_args);
        if result.is_ok(){
            println!("Done");
            return;
        }
    }
    commands["help"](vec![]); 
}
