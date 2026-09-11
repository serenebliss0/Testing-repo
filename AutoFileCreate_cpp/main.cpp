#include <iostream>
#include <optional>
#include <string>
#include <fstream>
#include <filesystem>
#include <cstdlib>
#include <sstream>
#include <thread>
#include <chrono>

using namespace std;

//argument count, argument vector
int main(int argc, char* argv[]){

    cout << "Welcome to the C++ edition of AutoFileCreate\n";
    cout << "This program will help you create all the C++ source files you need for your project";

    std::optional<int> week;
    bool config = false;

    for (int i = 1; i < argc; i++) {
        std::string arg = argv[i];

        if (arg == "--config") {
            config = true;
        }
        else if (arg == "--week" && i + 1 < argc) {
            week = std::stoi(argv[++i]);
        }
    }

    //if user wants to open config.serenity
    if (config == true){
        
        //check if config exists
        if(std::filesystem::exists("config.serenity")){
            //add error handling later
            std::system("notepad.exe config.serenity");
        }
        else{
            cout << "config.serenity doesn't exist yet. Creating an empty one...";
            std::ofstream file("config.serenity"); //creates empty config

            std::system("notepad.exe config.serenity");
        }
        return 0; //stop program here if config mode
    }

    string mainPath; //load or create main path

    //check if config exists
    if(std::filesystem::exists("config.serenity")){
        //add error handling later
        std::ifstream file("config.serenity");

        std::stringstream buffer;
        buffer << file.rdbuf();
    
        mainPath = buffer.str();
    }
    else{
        cout << "Type in the directory path of your COS201 folder\n";
        cin.ignore();
        getline(cin, mainPath);

        std::ofstream file("config.serenity");
        file <<mainPath;
    }

    //WEEK selection
    if (!week){

        cout << "\nWhat week is this currently?" << '\n';
        cin >> *week;
    }

    string directoryPath = mainPath + "/week_" + std::to_string(*week);
    std::filesystem::create_directory(directoryPath);
    std::filesystem::current_path(directoryPath);

    cout << "How many practice files do you want to create?\n";
    unsigned int NumOfPracticeFiles;
    cin >> NumOfPracticeFiles;
    
    //each file contains this by default
    string programTemplate = R"(#include <iostream>

using namespace std;

void doSomething(){
    cout << "Hello World!";
    }

int main(){

    doSomething();
    

    return 0;

}
        
)";


    if (NumOfPracticeFiles == 0){
        cout << "No files to create\n";
    }
    else{
        for (int i = 1; i <= NumOfPracticeFiles; i++){
            string current_practice = "practice_" + std::to_string(i) + ".cpp";

            std::ofstream file(current_practice);

            file << programTemplate;

        }

        cout << "All practice directories have been created";
        std::this_thread::sleep_for(std::chrono::seconds(2));

    }

    cout << "\nNow how many project files do you desire?\n";

    unsigned int NumOfProjectFiles;
    cin >> NumOfProjectFiles;

    if (NumOfProjectFiles == 0){
        cout << "Oooh no projects this week\nGoodbyeeeee!!!!";
        return 0;
    }
    else{
        for (int i = 1; i <= NumOfProjectFiles; i++){
            string current_project = "project_" + std::to_string(i) + ".cpp";

            std::ofstream file(current_project);

            file << programTemplate;

        }

        cout << "All project files have been created";
    }

    std::this_thread::sleep_for(chrono::seconds(1));
    cout << '\n' << "Thanks for using my tool";
    cout << '\n' << "Made with <3 by Semire\nhttps://github.com/serenebliss0";

    cout << "\nThis program will auto self-destruct in 5 seconds";
    std::this_thread::sleep_for(chrono::seconds(5));

    return 1;
}
