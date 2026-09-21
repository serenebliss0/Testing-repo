#include <iostream>

using namespace std;

void passPointer(int* p){
    cout << "In the function" << endl;
    cout << "*p = " << *p << endl;
    cout << "p = " << p << endl;
}

int* add (int a, int b){
    static int c[0];
    c[0] =  a + b;

    return c;
}

int main(){
    // int num = 9;

    // passPointer(&num);

    // cout << "Outside function" <<endl;
    // cout << "num = " << num << endl;
    // cout << &num <<endl;

    int *p = NULL, num1 =2, num2 = 6;
    p = add(num1, num2);

    cout << "*p = " << *p << endl;
    cout << "p = " << p << endl;
    
}