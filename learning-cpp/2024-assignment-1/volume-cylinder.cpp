#include <iostream>

using namespace std;

constexpr double PI = 3.142;

inline double calculateCylinderVolume(double height, double radius){
    return (PI * height * (radius*radius));
}

int main(){
    double radius, height;

    cout << "Enter the radius of your cylinder\n";
    cin >> radius;

    cout << "Now enter the height of the cylinder\n";
    cin >> height;

    cout << "The volume of your cylinder is " << calculateCylinderVolume(height, radius) << '\n';
}