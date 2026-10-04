#include <iostream>
#include <vector>
#include <string>
#include <cstdlib>
#include <ctime>

using namespace std;

//Constants
constexpr double lodgingDiscount = 0.05;
constexpr double registrationDiscount = 0.03;

struct studentDetails{
    string studentName;
    bool isPAUStudent;
    unsigned int courseChoice;
    unsigned int locationChoice;
};

struct courses{
    unsigned int serialNumber;
    string courseName;
    unsigned int days;
    int registrationFee;
};

struct accommodation{
    unsigned int serialNumber;
    string location;
    int lodgingFee;
};

vector <courses> availableCourses = {};
vector <accommodation> availableLocations = {};

courses photography;
courses painting;
courses fishFarming;
courses baking;
courses publicSpeaking;

void createCourses(){
    photography = {1, "Photography", 3, 10000};
    painting = {2, "Painting", 5, 8000};
    fishFarming = {3, "Fish Farming", 7, 15000};
    baking = {4, "Baking", 5, 13000};
    publicSpeaking = {5, "Public Speaking", 2, 5000};

    availableCourses.push_back(photography);
    availableCourses.push_back(painting);
    availableCourses.push_back(fishFarming);
    availableCourses.push_back(baking);
    availableCourses.push_back(publicSpeaking);
}

accommodation houseA;
accommodation houseB;
accommodation houseC;
accommodation houseD;
accommodation houseE;

void createLocations(){
    houseA = {1, "Camp House A", 10000};
    houseB = {2, "Camp House B", 2500};
    houseC = {3, "Camp House C", 5000};
    houseD = {4, "Camp House D", 13000};
    houseE = {5, "Camp House E", 5000};

    availableLocations.push_back(houseA);
    availableLocations.push_back(houseB);
    availableLocations.push_back(houseC);
    availableLocations.push_back(houseD);
    availableLocations.push_back(houseE);
}

void printCourses(){
    for (auto course: availableCourses){
        cout << course.serialNumber << " " << course.courseName << " " << course.days << " " << course.registrationFee << "\n";
    }
}

void printLocations(){
    for (auto lodge: availableLocations){
        cout << lodge.serialNumber << " " << lodge.location << " " << lodge.lodgingFee << "\n";
    }
}

studentDetails collectInputs(){
    string studentName;
    bool isPAUStudent;
    unsigned int courseChoice;
    unsigned int locationChoice;

    cout << "Enter full name:\n";
    getline(cin, studentName);

    cout << "IS PAU student? (1=true, 0=false):\n";
    cin >> isPAUStudent;

    (isPAUStudent != 1 || isPAUStudent != 0) ? cout << "" : cout << "Invalid value entered!\n";

    printCourses();

    cout << "Enter course (1-5):\n";
    cin >> courseChoice;

    printLocations();

    cout << "Enter location (1-5)\n";
    cin >> locationChoice;

    return {studentName, isPAUStudent, courseChoice, locationChoice};
}

int getLodgingFee(int locationChoice){

    int lodgingPerDay;

    if (locationChoice == 1){
        lodgingPerDay = houseA.lodgingFee;
    }
    else if(locationChoice == 2){
        lodgingPerDay = houseB.lodgingFee;
    }
    else if(locationChoice == 3){
        lodgingPerDay = houseC.lodgingFee;
    }
    else if(locationChoice == 4){
        lodgingPerDay = houseD.lodgingFee;
    }
    else if(locationChoice == 5){
        lodgingPerDay = houseE.lodgingFee;
    }
    else{
        cout << "Invalid Lodge was chosen\n";
        return 0;
    }

    return lodgingPerDay;
}

vector <int> mapCourseToChoice(int courseChoice){

    int courseDays;
    int registrationFee;

    if (courseChoice == 1){
        courseDays = photography.days;
        registrationFee = photography.registrationFee;
    }
    else if(courseChoice == 2){
        courseDays = painting.days;
        registrationFee = painting.registrationFee;
    }
    else if(courseChoice == 3){
        courseDays = fishFarming.days;
        registrationFee = fishFarming.registrationFee;
    }
    else if(courseChoice == 4){
        courseDays = baking.days;
        registrationFee = baking.registrationFee;
    }
    else if(courseChoice == 5){
        courseDays = publicSpeaking.days;
        registrationFee = publicSpeaking.registrationFee;
    }
    else{
        cout << "Invalid Course was chosen\n";
        
    }
    return {courseDays, registrationFee};
}

void processInputs(){
    studentDetails student= collectInputs();

    int days = mapCourseToChoice(student.courseChoice)[0];
    int registrationFee = mapCourseToChoice(student.courseChoice)[1];
    int lodgingPerDay = getLodgingFee(student.locationChoice);

    int lodgingCost = lodgingPerDay * days;
    int total = registrationFee + lodgingCost;

    if(student.isPAUStudent && ((student.locationChoice == 2) || (student.locationChoice == 3))){
        lodgingCost -= (lodgingCost * lodgingDiscount); //discount   
    }

    if(days > 5 || registrationFee > 12000){
        registrationFee -= (registrationFee * registrationDiscount);
    }

    total = registrationFee + lodgingCost;


    int randomNumber = rand() % 100 + 1;



    cout << "Name: " << student.studentName << " " << "(PAU student: )" << student.isPAUStudent ? cout << "Yes\n" : cout << "No\n";
    cout << "Course: " << "Course Name" << " " << "Days: " << days << '\n';
    cout << "Registration: N" << registrationFee << '\n';
    cout << "Lodging: N" << lodgingPerDay << " x " << days << "=" << lodgingCost << '\n';
    cout << "Total: N" << total;

    if(randomNumber == 7 || randomNumber == 77){
        total -= 500; //500 naira promotion
        cout << "Random Draw: " << randomNumber << " " << "Promo applied: N500\n";
    }
}




int main(){

    srand(time(0));

    createCourses();
    createLocations();

    processInputs();
}