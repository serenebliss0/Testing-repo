#include <iostream>

using namespace std;

int doBinarySearch(int array[], int size, int value){
    int low = 0;
    int high = (size - 1);

    while (low <= high){
    int mid = (low + high) /2;

        if (array[mid] == value){
            return mid;
        }
        else if (array[mid] < value){
            low = mid + 1;
        }
        else if (array[mid] > value){
            high = mid -1;
        }
        else{
            cout << "Value not found";
            return 0;
        }
    }
}

int main(){
    int numbers[] = {3, 8, 12, 17, 24, 31, 45, 52};
    int size = 8;

    int position = doBinarySearch(numbers, size, 45);

    cout << "The value is at position " << position;
}