#include <iostream>
#include <string>

using namespace std;

//@TODO
// Write a Rust program that takes a student’s name 
// and three test scores as input, 
// calculates the average, 
// and prints both the student’s name and 
// grade using the scale below: 
// Requirements: 
// ● Use variables, standard input, and if..else if..else statements. 
// ● Round the average to two decimal places. 
// ● Ensure invalid scores (less than 0 or greater than 100) display an error message. 

constexpr int testCount = 3;

double calculateAverage(int score1, int score2, int score3){
    return static_cast<double>((score1 + score2 + score3)) / testCount;
}

inline char assignGrade(double average){
    if(average >= 70){
        return 'A';
    }
    else if(average >= 60 && average <= 69){
        return 'B';
    }
    else if(average >= 50 && average <= 59){
        return 'C';
    }
    else if(average >= 45 && average <= 49){
        return 'D';
    }
    else if(average >= 0 && average <= 44){
        return 'F';
    }
    else{
        return '?';
    }
}

void handleScoreInput(unsigned int* score1, unsigned int* score2, unsigned int* score3){

    int scores[testCount];

    for (int i =0; i < testCount; i++){
        cout << "Enter score " << i << '\n';
        cin >> scores[i];

        if(scores[i] > 100 || scores[i] < 0){
            cout << "Invalid score";
            break;
        }

    }

    *score1 = scores[0] ;
    *score2 = scores[1] ;
    *score3 = scores[2] ;




}

int main(){

    string studentName;
    unsigned int score1, score2, score3;

    cout << "Please enter the student's name:\n";
    getline(cin, studentName);

    handleScoreInput(&score1, &score2, &score3);

    double average = calculateAverage(score1, score2, score3);

    cout << "\nStudent Name: " << studentName << '\n';
    cout << "Grade is: " << assignGrade(average) << "\n";
    
    return 0;
}

