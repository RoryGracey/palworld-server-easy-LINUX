use clap::Parser;
use std::process::Command;
use std::process::{self, Output};

#[derive(Parser)]
struct Cli {
    option: String,
}

fn install() {
    Command::new("apt")
        .arg("update")
        .status()
        .expect("Error apt updating ");
    Command::new("apt")
        .arg("dist-upgrade")
        .status()
        .expect("Error with dist-upgrade");
    Command::new("apt")
        .args(["install", "software-properties-common"])
        .status()
        .expect("Error installing software-properties-common");

    let device_check: std::process::Output = Command::new("uname")
        .arg("-a")
        .output()
        .expect("Unable to retrieve device type");

    let mut is_ubuntu = false;
    let uname_output = String::from_utf8_lossy(&device_check.stdout);
    if uname_output.contains("Ubuntu") {
        is_ubuntu = true;
    } else {
        is_ubuntu = false;
    }

    if is_ubuntu {
        let steam_cmd_install = Command::new("sh")
            .arg("-c")
            .arg("sudo apt install -y software-properties-common && sudo apt-add-repository -y main universe restricted multiverse && sudo dpkg --add-architecture i386 && sudo apt update && sudo apt install -y steamcmd")
            .output()
            .expect("Unable to install SteamCMD");
        if steam_cmd_install.status.success() {
            println!("SteamCMD successfully installed!")
        } else {
            println!("Unable to install SteamCMD");
            process::exit(1);
        }
    } else {
        let steam_cmd_install = Command::new("sh")
            .arg("-c")
            .arg("apt install software-properties-common && apt-add-repository non-free && dpkg --add-architecture i386 && apt update && apt install steamcmd")
            .output()
            .expect("Unable to install steam CMD");
        if steam_cmd_install.status.success() {
            println!("SteamCMD successfully installed!")
        } else {
            println!("Unable to install SteamCMD");
            process::exit(1);
        }
    }

    let create_user = Command::new("su")
        .arg("-c")
        .arg("useradd -m steam && passwd steam")
        .output();

    match create_user {
        Ok(output) => {
            if output.status.success() {
                println!("Created steam user!");
            } else {
                eprintln!(
                    "Failed to create steam user: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error executing command: {}", e);
            if !e.to_string().contains("'steam' already exists") {
                process::exit(1);
            }
        }
    }

    let change_user = Command::new("sudo")
        .arg("-u")
        .arg("steam")
        .arg("-s")
        .output()
        .expect("Failed to change user.");
    if change_user.status.success() {
        println!("Changed to steam user!");
    } else {
        println!("Unable to change to steam user.");
        process::exit(1);
    }

    let install_palworld = Command::new("su").arg("-c").arg("cd /home/steam && /usr/games/steamcmd +login anonymous +app_update 2394010 validate +quit").output().expect("Unable to run palworld install.");

    if !install_palworld.status.success() {
        println!("Couldn't install palworld files.");
        process::exit(1);
    }

    let check_install = Command::new("su")
        .arg("-c")
        .arg("if test -d /home/steam/.steam ; then clear ; echo 'TRUE'; else clear ; echo 'FALSE'; fi")
        .output()
        .expect("Unable to run test comamand.");
    if String::from_utf8_lossy(&check_install.stdout).contains("FALSE") {
        println!(".steam folder has had issues. It's likely https://github.com/A1RM4X/HowTo-Palworld/blob/main/README-no.steam.md this issue, but we currently don't support fixed for this automatically.");
        process::exit(1);
    }
}

fn main() {
    let args = Cli::parse();

    if args.option == "install" {
        install()
    }
}
