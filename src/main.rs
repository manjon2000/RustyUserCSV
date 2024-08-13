use std::fmt::{Debug};
use std::fs;
use std::path::PathBuf;
use std::string::ToString;
use clap::{Parser, Subcommand};

const PATH: &str = "./src/users.csv";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    All,
    Find {
        id: String
    },
    Create {
        #[arg(short)]
        username: String,
        #[arg(short)]
        first_name: String,
        #[arg(short)]
        last_name: String
    },
    Edit{
        identify: String,
        #[arg(short)]
        username: Option<String>,
        #[arg(short)]
        first_name: Option<String>,
        #[arg(short)]
        last_name: Option<String>
    },
    Delete {
        id: String
    }
}

#[derive(Debug)]
pub struct Users {
    pub identify: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub line: usize,
}

fn main() -> Result<(),String>{
    let users: Vec<Users> = read_file_and_map_users().expect("Error");
    let args = Args::parse();
    match args.cmd {
        Commands::All => {
            print_all_users(&users);
        },
        Commands::Create {username, first_name, last_name} => {
            created_user(
                &users,
                &username,
                &first_name,
                &last_name
            ).expect("ERROR CREATED OR WRITE IN FILE");
        },
        Commands::Find {id} => {
            let find_user = find_user_by_id(&id, &users);
            match find_user {
                Some(users) => { print_find_user(users) },
                None =>  { println!("Not found user") }
            }
        },
        Commands::Edit {identify, username, first_name, last_name} => {
            let user_exist = find_user_by_id(&identify,&users);

            if let Some(user) = user_exist {
                let fields = Users {
                    identify,
                    username: username.unwrap_or(user.username.clone()),
                    first_name: first_name.unwrap_or(user.first_name.clone()),
                    last_name: last_name.unwrap_or(user.last_name.clone()),
                    line: user.line
                };
                let edited_user = edited_line_file(fields).expect("Could not edit user");

              println!("{:?}", edited_user);


            }else if user_exist.is_none() {
                println!("Not found user")
            }

        },
        Commands::Delete { id } => {
            let line_user = get_line(&id, &users);
            match line_user {
                Ok(line) => { deleted_line_file(line).expect("Not Found User"); },
                Err(error) => { println!("Code: {:?} - Not found User", error); }
            }
        }
    }
    Ok(())
}

fn increment_index(current_index: &str) -> String {
    let transform_int = current_index.parse::<i32>().unwrap();
    (transform_int + 1).to_string()
}
fn formating_user_to_string(user: Users) -> String {
    format!("{};{};{};{}\n", &user.username, &user.identify, &user.first_name, &user.last_name)
}
fn write_file(lines: Vec<&str>) -> Result<(), String> {
    fs::write(PATH, lines.join("\n")).map_err(|x|x.to_string())?;
    Ok(())
}
fn read_file_and_map_users() -> Result<Vec<Users>,String>{
    let v = fs::read(PathBuf::from(PATH).as_path()).map_err(|x|x.to_string())?;
    let mut users: Vec<Users> = vec![];

    let content = String::from_utf8(v).expect("Error parse bytes to str");
    let all_lines = content.lines();
    for (index, line) in all_lines.skip(1).enumerate() {
        users.push(map_user_in_line(line, index+1)?);
    }
    Ok(users)
}
fn map_user_in_line(user_line: &str, number_line: usize) -> Result<Users,String> {
    let field= user_line.split(';').collect::<Vec<&str>>();

    Ok(
        Users{
            identify: field.get(1).map(|x|x.to_string()).ok_or("Error reading the identifier")?,
            username: field.first().map(|x|x.to_string()).ok_or("Error reading username")?,
            first_name: field.get(2).map(|x|x.to_string()).ok_or("Name reading error")?,
            last_name: field.get(3).map(|x|x.to_string()).ok_or("Error in the reading of the Last Name")?,
            line: number_line
        }
    )
}
fn created_user(ref_user: &[Users], username: &str, first_name: &str, last_name: &str) -> Result<(), String> {
    let last_index_users = ref_user.last().unwrap().identify.to_string();
    let index = increment_index(&last_index_users);
    let new_user: Users =  Users {
        identify: index,
        username: username.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        line: ref_user.len() + 1
    };
    
    let content = fs::read_to_string(PATH).map_err(|x|x.to_string())?;
    let mut lines: Vec<&str> = content.lines().collect();
    let format_user_to_string = formating_user_to_string(new_user);
    lines.push(&format_user_to_string);
    write_file(lines).map_err(|x|x.to_string())?;

    Ok(())
}
fn find_user_by_id<'a>(id: &str, users: &'a [Users]) -> Option<&'a Users> {
    users.iter().find(|u|u.identify.eq(id))
}
fn edited_line_file(user: Users) -> Result<String, String> {
    let content = fs::read_to_string(PATH).map_err(|x|x.to_string())?;
    let mut lines: Vec<&str> = content.lines().collect();

    if user.line > 0 && user.line <= lines.len() {
        let mut fields_row: Vec<&str> = lines[user.line].split(";").collect::<Vec<&str>>();

        fields_row[0] = &user.username;
        fields_row[2] = &user.first_name;
        fields_row[3] = &user.last_name;

        let formating_string = fields_row.join(";");
        lines[user.line] = &formating_string;
        write_file(lines).map_err(|x|x.to_string())?;
    }
    Ok("Successfully edited user".parse().unwrap())
}
fn deleted_line_file(line: usize) -> Result<(), String> {
    let content = fs::read_to_string(PATH).map_err(|x|x.to_string())?;
    let mut lines: Vec<&str> = content.lines().collect();
    if line > 0 && line <= lines.len() {
        lines.remove(line);
    }
    write_file(lines).map_err(|x|x.to_string())?;
    Ok(())
}
fn get_line(id: &str, users: &[Users]) -> Result<usize, i32> {
    let find_user = users.iter().find(|u| { u.identify.eq(id) });
    match find_user {
        None => { Err(404) },
        Some(user) => { Ok(user.line) }
    }
}
fn print_all_users(users: &[Users]) {
    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
        "Identify", "Username", "First Name", "Last Name"
    );
    for user in users.iter() {
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            user.identify,
            user.username,
            user.first_name,
            user.last_name,
        );
    }
}
fn print_find_user(user: &Users) {
    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
        "Identify", "Username", "First Name", "Last Name"
    );

    println!(
        "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
        user.identify,
        user.username,
        user.first_name,
        user.last_name,
    );
}