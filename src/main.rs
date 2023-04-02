use std::fs::rename;
use std::os::windows::fs::symlink_dir;
use std::path::{Component, Components};

fn main() {
    let mut args = std::env::args();
    let _exe_path = args.next();
    let dir_to_move = args.next();
    let dest_dir = args.next();
    if dir_to_move.is_none() || dest_dir.is_none() {
        panic!(
            r#"\n\nUsage: {} <path/to/dir/to/clean> <path/to/dest/dir>\n\n"#,
            _exe_path.unwrap()
        );
    }

    let dir_to_move = std::path::Path::new(dir_to_move.as_ref().unwrap());
    let dest_dir = std::path::Path::new(dest_dir.as_ref().unwrap());

    // append source dir name to dest dir
    let dir_to_move_name = dir_to_move
        .components()
        .next_back()
        .expect("Failed to get directory name")
        .as_os_str()
        .to_str()
        .expect("Failed to convert os str to str");
    let dest_dir_including_source_dir_name = dest_dir.join(dir_to_move_name);
    let dest_dir_including_source_dir_name = dest_dir_including_source_dir_name.to_str().unwrap();

    // move dir to clean to dest dir
    // let path_dir_to_move = std::path::Path::new(&dir_to_move);
    println!(
        "Moving [{}] to [{}] ...",
        dir_to_move_name, dest_dir_including_source_dir_name
    );

    rename(&dir_to_move, dest_dir_including_source_dir_name).expect(&format!(
        "\n\nfailed to move directory [{}] to [{}]\n\n",
        dir_to_move.to_str().unwrap(),
        dest_dir_including_source_dir_name
    ));

    // create a link to the dir moved to the dest dir
    println!(
        "Creating symlink [{}] -> [{}] ...",
        dir_to_move.to_str().as_ref().unwrap(),
        dest_dir_including_source_dir_name
    );
    symlink_dir(dest_dir_including_source_dir_name, dir_to_move).expect("Failed to create symlink");
}