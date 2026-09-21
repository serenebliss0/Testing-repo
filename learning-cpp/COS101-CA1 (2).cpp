#include <iostream>
#include <cmath>

using namespace std;

//TODO
// A customer deposits a sum of money P into a bank at an 
// interest rate R (in %) for T years. 
// Write a Rust program that calculates the 
// compound interest and total amount using the 
// formula: A = P \times (1 + R/100)^T 
// Program Requirements: 
// ● Accept values for P, R, and T using standard input. 
// ● Use variables, type casting, and float operations correctly. 
// ● Display the compound interest and total amount neatly. 
// ● Use loop or while to allow the user to calculate for more than one customer 
// (until they type "n" to stop). 

double calculateAmount(double principal, double rate, double time){
    return (principal * powf((1.0 + (rate/ 100.0)), time));

}

int main(){

    double principal, rate, time;

    cout << "Please enter your principal amount\n";
    cin >> principal;

    cout << "Enter your rate (in %)\n";
    cin >> rate;

    cout << "Enter your time (in years)\n";
    cin >> time;

    double amount = calculateAmount(principal, rate, time);

    cout << "The total amount is " << amount << "\n";
    cout << "The compound interest is " << (principal + amount);
    
}