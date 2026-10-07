#include <iostream>
#include <vector>

using namespace std;

//TODO
//Solve a simultaneous equation
    // a_1x+b_1y=c_1
    // a_2x+b_2y=c_2

double a1, b1, c1, a2, b2, c2;
int solveEquations();

void collectInputs(){
    cout << "Enter a value for a1\n";
    cin >> a1;
    cout << "Enter a value for b1\n";
    cin >> b1;
    cout << "Enter a value for c1\n";
    cin >> c1;
    cout << "Enter a value for a2\n";
    cin >> a2;
    cout << "Enter a value for b2\n";
    cin >> b2;
    cout << "Enter a value for c2\n";
    cin >> c2;
}

void handleDisplay(){
    cout << "Enter equations in the form\n";
    cout << "a_1x+b_1y=c_1\n";
    cout << "or a_2x+b_2y=c_2\n";

    collectInputs();
    solveEquations();

}

int solveEquations(){
    double denom = ((a1*b2) - (a2*b1));

    if(denom == 0){
        cout << "No unique solutions\n";
        return 0;
    }

    double x = ((c1* b2) - (c2*b1)) / denom;
    double y = ((a1*c2) - (a2*c1)) / denom;

    cout << "x is " << x << '\n';
    cout << "y is " << y << '\n';
}

int main(){
    handleDisplay();
}