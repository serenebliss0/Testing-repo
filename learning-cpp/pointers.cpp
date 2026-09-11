#include <iostream>

using namespace std;

void change_me(int x) {
    x = 50;
}

int main(){
    int x = 27;
    
    cout << &x;

    int *p = &x;

    cout << "\np is " << p;

    cout << "\nThe value at p is " << *p;

    *p = 50;

    cout << "\nThe value at p is now " << *p;

    int numbers[5] = {10,20,30,40,50};

    int *q;

    q = numbers; //here q is pointing to the first element in that array

    cout << "\nq is " << *q;

    q++; //this moves to the next element in that array
    //since q is an int, we're actually incrementing the addr by 4 bytes

    cout << "\nq is " << *q;


    char a = 'z';

    cout << "\n" << " a is " << a;
    char *b = &a;

    cout << "\n" << " b is " << *b;

    char **c = &b;

    cout << "\n" << " c is " << c;

    //The number of * tells us how many times we need to dereference
    int y = 10;
    void *r = &y;

    //Lets tell cpp to treat the address as an integer

    cout << "\nr is " <<  *(static_cast<int *>(r));

    int *s = nullptr;

    if (s != nullptr){
        cout << *s;
    }
    else{
        cout << "s is a null pointer";
    }

    cout << "The address of s is 0x" <<  std::hex <<(uintptr_t)&s << '\n';
    //when you use std::hex make sure to switch back to dec!!!!!

    cout << std::dec;
    int t = 10;
    change_me(t);

    cout << "\nvalue of t after function: " << t << '\n';
}



