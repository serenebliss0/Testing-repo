#include <iostream>

using namespace std;

inline double convertKelvinToFahrenheit(double kelvinTemp){
    return (((kelvinTemp - 273.15) * 9 ) / 5) + 32;
}

int main(){
    cout << "Enter a temperature in kelvin\n";

    double kelvinTemperature;
    cin >> kelvinTemperature;

    cout << kelvinTemperature << " in fahrenheit is " << convertKelvinToFahrenheit(kelvinTemperature) << '\n';
    
}