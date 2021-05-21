use structopt::StructOpt;
use std::fs::{self, DirEntry};
use std::collections::HashMap;
use std::os::linux::fs::MetadataExt;
use users::{get_user_by_uid, get_current_uid, get_current_username, get_group_by_gid, get_current_groupname, get_current_gid};
use colored::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "./")]
    path: String,
}

struct FileDetails {
    is_file: bool,
    file_name: String,
    file_owner: String,
    file_owner_id: u32,
    group_owner: String,
    group_owner_id: u32,
    owner: String,
    group: String,
    all_users: String,
}

fn get_permissions_hashmap() -> HashMap<String, String> {

    let mut permissions_hm = HashMap::new();
    permissions_hm.insert("0".to_string(), "---".to_string());
    permissions_hm.insert("1".to_string(), "--x".to_string());
    permissions_hm.insert("2".to_string(), "-w-".to_string());
    permissions_hm.insert("3".to_string(), "-wx".to_string());
    permissions_hm.insert("4".to_string(), "r--".to_string());
    permissions_hm.insert("5".to_string(), "r-x".to_string());
    permissions_hm.insert("6".to_string(), "rw-".to_string());
    permissions_hm.insert("7".to_string(), "rwx".to_string());

    return permissions_hm;
}

fn transform_octal_permissions_to_human_readable(oct_permission: String) -> String {

    let permissions_hm = get_permissions_hashmap();

    let symbolic_permissions: String = permissions_hm[&oct_permission].to_string();
    let mut permissions: String = String::new();

    for i in symbolic_permissions.chars() {
        match i {
            'r' => permissions.push_str("read"),
            'w' => if permissions.is_empty() {permissions.push_str("write")} else {permissions.push_str(", write")},
            'x' => if permissions.is_empty() {permissions.push_str("execute")} else {permissions.push_str(", execute")},
            _=> permissions.push_str("")
        }
    }

    if permissions.is_empty() {
        permissions.push_str("None");
    }

    return permissions;
}

fn get_file_permissions(entry: DirEntry) -> FileDetails {

    let entry_metadata = entry.metadata().unwrap();
    let entry_permission_mode = format!("{:o}", entry_metadata.st_mode());
    let mut permissions = entry_permission_mode.chars();

    permissions.next();
    permissions.next();

    let file_permissions: FileDetails = FileDetails {

        is_file: entry_metadata.is_file(),
        file_name: entry.file_name().into_string().unwrap(),
        file_owner: get_user_by_uid(entry_metadata.st_uid()).unwrap().name().to_str().unwrap().to_string(),
        file_owner_id: entry_metadata.st_uid(),
        group_owner: get_group_by_gid(entry_metadata.st_gid()).unwrap().name().to_str().unwrap().to_string(),
        group_owner_id: entry_metadata.st_gid(),
        owner: transform_octal_permissions_to_human_readable(permissions.next().unwrap().to_string()),
        group: transform_octal_permissions_to_human_readable(permissions.next().unwrap().to_string()),
        all_users: transform_octal_permissions_to_human_readable(permissions.next().unwrap().to_string()),

    };

    return file_permissions;
}

fn get_user_permission(permissions: FileDetails) -> String {
    
    let current_uid = get_current_uid();

    if permissions.file_owner_id == current_uid && permissions.group_owner_id == get_current_gid() {
        if permissions.owner.len() >= permissions.group.len() {
            return permissions.owner;
        }
        else {
            return permissions.group;
        }
    }
    else if permissions.file_owner_id == current_uid {
        return permissions.owner;
    }
    else if permissions.group_owner_id == get_current_gid() {
        return permissions.group;
    }
    else {
        return permissions.all_users;
    }

}

fn print_permissions(permissions: FileDetails) {

    println!("{} name: {}, Owner: {}(uid={}), Owner group: {}(gid={})", if permissions.is_file {"File"} else {"Directory"}, permissions.file_name.normal().bold(), permissions.file_owner.normal().bold(), permissions.file_owner_id, permissions.group_owner.normal().bold(), permissions.group_owner_id);
    println!("Owner: {}", permissions.owner.green());
    println!("Group: {}", permissions.group.blue());
    println!("All users: {}", permissions.all_users.red());
    println!("The logged in user({}), part of the group \"{}\" can: {}\n", get_current_username().unwrap().to_str().unwrap().to_string(), get_current_groupname().unwrap().to_str().unwrap().to_string(), get_user_permission(permissions).yellow());

}

fn main() {

    let args = Cli::from_args();

    for entry in fs::read_dir(args.path).unwrap() {

        let permissions: FileDetails = get_file_permissions(entry.unwrap());
        print_permissions(permissions);

    }

}