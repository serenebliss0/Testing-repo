#include <iostream>
#include <string>

using namespace std;

inline string convertNumberToTime(unsigned int number){

    string convertedTime;

    int hours = number / 60;
    int minutes = number % 60;

    convertedTime += to_string(hours);

    convertedTime += " : ";

    if(minutes <=9){
        //append zero if digits less than 2
        convertedTime += ("0" + to_string(minutes));
    }
    else{
        convertedTime += to_string(minutes);
    }

    return convertedTime;
}

int main(){
    cout << "Enter a number to convert to hh:mm\n";
    unsigned int number;
    cin >> number;

    cout << "Your number in hh:mm is " << convertNumberToTime(number) << '\n';
}