#include <iostream>
#include <cstdlib>
#include <ctime>
#include <map>
#include <string>
#include <optional>
#include <iomanip>

using namespace std;

//TODO
// Inputs
// Ask the user for:
// 1. Full name (string)
// 2. JAMB score (0–400, int)
// 3. WAEC average (0–100, double)
// 4. Age (in years, int)
// 5. First-choice PAU? (1/0 → store in bool)
// 6. Any disciplinary record? (1/0 → store in bool)
// 7. Hostel preference (1–3):
// o 1 = Main Hostel (₦180,000)
// o 2 = Annex Hostel (₦120,000)
// o 3 = Day Student (₦0)
// Use bool variables for the yes/no items.
// Fixed Values
// • Base tuition: ₦1,500,000
// • Hostel fees (per session): as above.

//Constants
constexpr double baseTuition = 1500000;

struct StudentDetails{
    string fullName;
    int jambScore;
    double waecScore;
    int age;
    bool firstChoice;
    bool disciplinaryStatus;
    int hostelPreference;
    string hostelName = "Null";
    double tuitionFees = 1500000;
    optional<bool> isAdmitted;
    int hostelFees = 0;
    string hostelStatus;
    double totalPayable = 0;
};

struct Hostels{
    string Name;
    const int price = 0;
};

map <int, Hostels> hostels = {
    {1, {"Main Hostel", 180000}},
    {2, {"Annex Hostel", 120000}},
    {3, {"Day Student", 0}}
};



StudentDetails collectInputs(){
    StudentDetails student;

    cout << endl;

    //Full name
    cout << "Enter your name: ";
    getline(cin, student.fullName);

    //JAMB
    do{
        cout << "Enter JAMB score: ";
        cin >> student.jambScore;
    }
    while(student.jambScore < 0 || student.jambScore>400);

    //WAEC
    do{
        cout << "Enter WAEC average: ";
        cin >> student.waecScore;
    }
    while(student.waecScore <0 || student.waecScore>100);

    //Age
    do{
        cout << "Enter age: ";
        cin >> student.age;
    }
    while(student.age < 0);


    cout << "Is PAU your first choice? (Enter 1-yes, 0-no): ";
    cin >> student.firstChoice;

    cout << "Any disciplinary record? (Enter 1-yes, 0-no): ";
    cin >> student.disciplinaryStatus;


    for (auto hostel : hostels){
        cout << hostel.first << " " << hostel.second.Name << hostel.second.price << "\n";
    }

    do{
        cout << "Enter 1,2,3 for your hostel";
        cin >> student.hostelPreference;
        student.hostelName = hostels[student.hostelPreference].Name;
    }
    while(student.hostelPreference > 4 || student.hostelPreference < 1);

    return student;
}

optional<bool> admissionStatus(StudentDetails &student){

    optional<bool> isAdmitted;

    if((student.jambScore >= 200 && student.waecScore >= 60) && student.firstChoice){
        student.isAdmitted = true;
        isAdmitted = true;
    }
    else if(student.jambScore >= 200 && student.waecScore >=70){
        student.isAdmitted = true;
        isAdmitted = true;
    }
    else if(student.disciplinaryStatus || student.age < 15){
        student.isAdmitted = true;
        isAdmitted = false;
    }
    else{
        student.isAdmitted = nullopt;
        isAdmitted = nullopt;
    }

    return isAdmitted;
}

double decideScholarship(StudentDetails &student){
    optional<bool> isAdmitted = admissionStatus(student);

    double scholarshipAmount = 0.0; //convert to % later

    if(!isAdmitted){
        return 0.0;
    }
    else{

        if(student.jambScore >= 320){
            scholarshipAmount += 0.3;
            student.tuitionFees -= (student.tuitionFees * scholarshipAmount);
        }
        else if(student.jambScore >= 280){
            scholarshipAmount += 0.2;
            student.tuitionFees -= (student.tuitionFees * scholarshipAmount);
        }
        else if(student.jambScore >= 240){
            scholarshipAmount += 0.1;
            student.tuitionFees -= (student.tuitionFees * scholarshipAmount);
        }
        else{} //0% discount. Do nothing
    }

    //Bonus scholarship
    if(scholarshipAmount <= 0.35){
    if(student.waecScore >= 80 && student.firstChoice){
        scholarshipAmount += 0.05;
        student.tuitionFees -= (student.tuitionFees * scholarshipAmount);
    }
}

return scholarshipAmount;
}

void doHostelAssignment(StudentDetails &student){
    if(student.isAdmitted && !student.disciplinaryStatus){
        student.hostelFees = hostels[student.hostelPreference].price;
    }
    else if(!student.isAdmitted.has_value()){
        student.hostelFees = 0; //On Hold
        student.hostelStatus = "On Hold";
    }
    else if(!student.isAdmitted){
        student.hostelFees = 0;
        student.hostelStatus = "Rejected";
    }
    else{} //do nothing
}

bool isNumberPrime(int r){
    bool isPrime =
    r == 2  || r == 3  || r == 5  || r == 7  ||
    r == 11 || r == 13 || r == 17 || r == 19 ||
    r == 23 || r == 29 || r == 31 || r == 37 ||
    r == 41 || r == 43 || r == 47 || r == 53 ||
    r == 59 || r == 61 || r == 67 || r == 71 ||
    r == 73 || r == 79 || r == 83 || r == 89 ||
    r == 97;

    return isPrime;
}

int generateRandomMerit(StudentDetails &student){
    int randomNumber = rand() % 100 + 1;

    if(isNumberPrime(randomNumber) && student.isAdmitted){
        student.tuitionFees -= 50000;
    }
    return randomNumber;
}

void calculateTotal(StudentDetails &student){
    student.totalPayable = student.tuitionFees + student.hostelFees;
    //merit grant and scholarship already done
}

void printOutput(StudentDetails &student){

    calculateTotal(student);

    int randomNumber = generateRandomMerit(student);


    cout << "Name: " << student.fullName << '\n';
    cout << "Admission Status: ";

    if(!student.isAdmitted.has_value()){
        cout << "Pending\n";
    }
    else if(student.isAdmitted){
        cout << "Admitted\n";
    }
    else{
        cout << "Rejected\n";
    }

    cout << "Scholarship: " << (decideScholarship(student) * 100) << "%" << " " << "Tuition After Scholarship " << student.totalPayable << '\n';
    cout << "Hostel: ";

    if(student.hostelStatus == "On Hold"){
        cout << "On Hold\n";
    }
    else if(student.hostelStatus == "Rejected"){
        cout << "Not Applicable";
    }
    else{
        cout << student.hostelName << " " << "Fee: N" << student.hostelFees << '\n';
    }

    cout << "Random Draw: " << randomNumber << " ";

    if(isNumberPrime(randomNumber)){
        cout << "Merit Grant: N" << "50000\n";
    }

    cout << "\nTOTAL PAYABLE: N" << fixed << setprecision(2) << student.totalPayable;
}

int main(){
    srand(time(0));
    StudentDetails student = collectInputs();
    printOutput(student);
}