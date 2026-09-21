#include <iostream>
#include <vector>
#include <iomanip>
#include <cctype>
using namespace std;

constexpr int discountPercent = 7; //amount to discount for orders over 500,000

struct Item{
    string itemName;
    char itemCode;
    unsigned int itemPrice;
};


int main(){

    //available items
    Item laptop = {"Laptop", 'L', 550000};
    Item monitor = {"Monitor", 'M', 120000};
    Item keyboard = {"Keyboard", 'K', 15000 };
    Item headset = {"Headset", 'H', 25000};

    vector <Item> storeMenu = {laptop, monitor, keyboard, headset};
    vector <Item> userCart;

    char userChoice;

    do{
    //Table headers
    cout << "Code" << '\t' << "Item" << '\t' << "Price" << '\n';

    for (auto item: storeMenu){
        cout << item.itemCode << '\t' << item.itemName << '\t' << item.itemPrice << '\n';
    }
    
    cout << "Enter an item code from the menu\n";

    char userItemCode;
    unsigned int quantity;

    cin >> userItemCode;
    userItemCode = toupper(userItemCode);
    

    switch (userItemCode){

        case 'L': userCart.push_back(laptop); break;
        case 'M': userCart.push_back(monitor); break;
        case 'K': userCart.push_back(keyboard); break;
        case 'H' : userCart.push_back(headset); break;

        default: 
        cout << "The item code you entered does not exist\n";
        return 0;
    };

    cout << "How many do you want to purchase?\n";
    
    cin >> quantity;

    double totalCost = userCart[0].itemPrice * quantity;

    if (totalCost > 500000){
        totalCost = static_cast<double>(totalCost - (totalCost * (discountPercent / 100.0)));
    }

    cout << "The final amount payable is " << fixed << setprecision(2) << totalCost;

    cout <<"\nDo you want to make another order?\n";
    cout << "Type `y` yes  or `n` to quit\n";

    cin >> userChoice;
    }
    while(userChoice !='n');

}