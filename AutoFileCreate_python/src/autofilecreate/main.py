import os;
import argparse;
from pathlib import Path;
import subprocess;
import time;
import sys;

appdata = os.environ.get("APPDATA")
if not appdata:
    raise Exception("APPDATA environment variable not found!")

APP_DATA = Path(appdata) / "AutoFileCreate"
APP_DATA.mkdir(parents=True, exist_ok=True)
config_file = APP_DATA / "config.serenity"

def create_config():
    config_file = APP_DATA/ "config.serenity"
    config_file.touch();
    print("Enter the directory path for your COS102 folder");
    directory_path = input();
    config_file.write_text(directory_path);

def main():
    parser = argparse.ArgumentParser(description="AutoFileCreate (Python) for Jupyter Notebook");
    parser.add_argument("--week", type=int, required=False);
    parser.add_argument("--config", action="store_true");

    args = parser.parse_args()

    print("Welcome to the python edition of AutoFileCreate"); # semire is still here
    print("This program will help you create all the jupyter notebooks you need for your project");

    # todo!("Check if usr wants to open config.serenity")

    if args.config:
        if config_file.exists():
            os.startfile(config_file);
    else:
        print("Config file doesn't exist yet! Creating a new one");
        create_config();
        # make sure it exists, if not create it


    directory_path = "";
    current_week = "";
    directory = "";

    # read the contents of the config and collect mainPath
    if config_file.exists():
            with open(config_file, "r") as config:
                directory_path = config.read().strip();
                dir = Path(directory_path);
            print("CONFIG PATH:", directory_path);
            
            if not dir.exists():
                print("The path in config does not exist!");
                print("Please enter the correct path to your COS102 folder in config.serenity");
                os.startfile("config.serenity");
                return;

            if not dir.is_dir():
                print("The path in config is not a directory!");
                print("Please update your COS102 directory in config.serenity");
                os.startfile("config.serenity");
                return;

    else:
        print("Config doesn't exist! Creating a new one");
        create_config();

    
    # check if user passes a value for week
    if args.week:
        current_week = args.week;
        try:
            os.chdir(directory_path);
        except FileNotFoundError:
            print("The directory specified in config does not exist");
            print("Run program with `--config` to edit your configuration file");
            return;

        directory = f"week_{current_week}";
        try:
            os.makedirs(directory, exist_ok=True);
        except OSError as lag:
            print(f"Failed to create dir. {lag}");
            return;

        try:
            os.chdir(directory);
        except FileNotFoundError:
            print("The directory specified in config does not exist");
            print("Run program with `--config` to edit your configuration file");
            return;

    else:

        while True:
            try:
                current_week = int(input("What week is it currently?"));
                break;
                
            except ValueError:
                print("Invalid value entered");

    try:
        os.chdir(directory_path);
    except FileNotFoundError:
        print("The directory specified in config does not exist");
        print("Run program with `--config` to edit your configuration file");
        return;

    directory = f"week_{current_week}";
    try:
        os.makedirs(directory, exist_ok=True);
    except OSError as lag:
        print(f"Failed to create folder: {lag}");
        return;

    try:
        os.chdir(directory);
    except FileNotFoundError:
        print("The directory specified in config does not exist");
        print("Run program with `--config` to edit your configuration file");
        return;
        
    print(os.getcwd());

    # ask for amt of practice, project files
    while True:
        try:
            num_practice_files = int(input("How many practice files do you want to create?"));
            break;
        except ValueError:
            print("Invalid value entered! Try again");

    if num_practice_files == 0:
        print("No practice files to create");
        print("Skipping now...");
        return;

    else:
        for i in range(1, num_practice_files + 1):
            Path(f"practice_{i}.ipynb").touch();
        
        time.sleep(0.5);
        print("All practice files have been created");
    
    # loop through and create
    while True:
        try:
            num_project_files = int(input("Now how many project files do you desire"));
            break;
        except ValueError:
            print("Invalid value entered. Try again!");
        
    if num_project_files == 0:
        print("Ooh no projects for this week...");
        print("Goodbye!");
        return;
    else:
        for i in range(1, num_project_files + 1):
            Path(f"project_{i}.ipynb").touch();
            
        print("All project files have been created");

if __name__ == "__main__":
    main();
    print("Thanks for using my tool");
    print("Made with <3 by Semire\nhttps://github.com/serenebliss0");

    print("This program will auto self-destruct in 5 seconds");
    time.sleep(5);