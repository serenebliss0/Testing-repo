#include <iostream>

using namespace std;

//constants
constexpr int dailyLimit = 20000;
constexpr unsigned int defaultUserPin = 4321;

struct User{
    double accountBalance = 50000;
    int currentPIN = defaultUserPin;
};

User createCustomer(){
    User user;
    return user;
}

bool verifyPin(int correctPIN){

    int attempts = 1;
    unsigned int enteredPIN;

    do{
    cout << "Enter your PIN: \n";
    cin >> enteredPIN;
    attempts++;
    if(attempts > 3){
        return false; //check later
    }
    }
    while((enteredPIN != correctPIN) && attempts <=3);

    return true;
}

void displayMenu(){
    cout << "\n======= ATM MENU =======\n";
    cout << "1. View Balance\n";
    cout << "2. Deposit Cash\n";
    cout << "3. Withdraw Cash\n";
    cout << "4. Reset PIN\n";
    cout << "5. Exit\n";
}

double viewBalance(double balance){
    cout << "Current Balance: N" << balance << '\n';

    return balance;
}

double depositMoney(double &balance){
    double depositAmount;

    do{
    cout << "How much do you want to deposit?\n";
    cin >> depositAmount;
    }
    while(depositAmount <0);

    return (balance + depositAmount);
}

double withdrawMoney(double &balance, double dailyLimit){

    double amount = 0.0;

    do{
        cout << "How much do you want to withdraw?\n";
        cin >> amount;
    }
    while (amount < 0);

    if (amount <= dailyLimit){
        if (amount <= balance){
            balance -= amount;
            cout << "Withdrawal of  " << amount << " successful\n";
        }
        else{
            cout << "Amount exceeds balance\n";
        }
    }
    else{
        cout << "Amount exceeds daily limit\n";
    }

    return balance;
}

int resetPIN(int currentPin){
    cout << "Enter your current PIN\n";

    int correctPIN = verifyPin(currentPin);
    int newPIN = currentPin;

    if(correctPIN){
        cout << "Enter a new PIN\n";
        cin >> newPIN;

        cout << "New Pin saved\n";
    }

    return newPIN;
}

int handleInputs(){
    User user = createCustomer();

    bool pinStatus = verifyPin(user.currentPIN);

    if (!pinStatus){
        cout << "Account Locked. Please contact your bank\n";
    }
    else{
        int userChoice;
        do{
        displayMenu();

        cout << "Enter your choice: ";
        cin >> userChoice;

        switch(userChoice){
            case 1 : viewBalance(user.accountBalance); break;
            case 2: user.accountBalance = depositMoney(user.accountBalance); break;
            case 3: withdrawMoney(user.accountBalance, dailyLimit); break;
            case 4: user.currentPIN = resetPIN(user.currentPIN); break;
            case 5: cout << "Thank you for using our ATM\n"; return 0;

            default:
                cout << "Invalid option. Try again."; displayMenu();
        }
    }
    while(userChoice != 5);

    }
}

int main(){
    handleInputs();
}
