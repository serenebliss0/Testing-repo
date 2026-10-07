#include <iostream>
#include <map>
#include <vector>
#include <optional>
#include <cctype>

using namespace std;

//TODO
// Quiz game criteria
//  The game should allow the user to enter character options from the keyboard.
//  The game should accept either a lower or an upper case characters for each option.
//  The game should retain the option entered by the user for each question.
//  The game should provide the user with an option to skip a question
//  The game should provide the user with an option to return to skipped questions
//  On completion of the quiz, the game should display the user’s score.
//  The game should provide the user with an option to view his or her script.

typedef struct{
    string questionContent;
    vector <string> options;
    char correctOption;
} Question;

//all questions
map<int, Question> questions{
    {1, {
        "Which of the following is the correct identifier?",
        {
            "$var_name",
            "VAR_123",
            "varname@",
            "None of the above"
        },
        'B'
    }},

    {2, {
        "Which of the following is the address operator?",
        {
            "@",
            "#",
            "&",
            "%"
        },
        'C'
    }},

    {3, {
        "Which of the following features must be supported by any programming language to become a pure object-oriented programming language?",
        {
            "Encapsulation",
            "Inheritance",
            "Polymorphism",
            "All of the above"
        },
        'D'
    }},

    {4, {
        "Which of the following refers to characteristics of an array?",
        {
            "An array is a set of similar data items",
            "An array is a set of distinct data items",
            "An array can hold different types of data",
            "None of the above"
        },
        'A'
    }},

    {5, {
        "If we stored five elements or data items in an array, what will be the index address or the index number of the array's last data item?",
        {
            "2",
            "3",
            "4",
            "5"
        },
        'C'
    }},

    {6, {
        "Which of the following is the correct syntax for declaring the array?",
        {
            "int array{};",
            "int array[5];",
            "Array[5];",
            "None of the above"
        },
        'B'
    }},

    {7, {
        "Which of the following is the correct syntax for accessing the first element?",
        {
            "array[2];",
            "array[0]",
            "Array[5];",
            "array[1]"
        },
        'B'
    }},

    {8, {
        "Which of the following gives the 4th element of the array?",
        {
            "array[2];",
            "array[3]",
            "Array[5];",
            "array[1]"
        },
        'B'
    }},

    {9, {
        "Which type of memory is used by an Array in C++ programming language?",
        {
            "Contiguous",
            "None-contiguous",
            "Both A and B",
            "Not mentioned"
        },
        'A'
    }},

    {10, {
        "Which one of the following is the correct definition of the \"is_array();\" function in C++?",
        {
            "It checks that the specified variable is of the array or not",
            "It checks that the specified array is of single dimension or not",
            "It checks that the array specified is of multi-dimension or not",
            "Both B and C"
        },
        'A'
    }}
};

vector<char> userAnswers(questions.size(), '\0'); //contains all the user's answers globally

//declare beforehand
void moveToPrevious();
void skipToNext();
int checkUserAnswers();
int displayResults();
void submitTest();


void printRules(){
    cout << "\tRules of the game\n";
    cout << "\t---------------------\n";
    cout << "You are expected to answer 10 questions\n";
    cout << "You can skip a question and return to it later\n";
    cout << "To answer a question enter the right option on the keyboard and press `Enter`\n";
    cout << "To skip a question press `N`\n";
    cout << "To go back to previous questions, press `P`\n";
    cout << "To submit press F\n";

    //Call print questions
}

int printWelcome(){
    cout << "\nWelcome to C++ quiz game\n";
    cout << "Press 1 to Start\n";
    cout << "Press 2 to exit\n";

    int userChoice;
    cin >> userChoice;

    switch(userChoice){
        case 1: printRules(); break;
        case 2: return 0;

        default: 
            cout << "You entered a wrong value"; printWelcome(); //better try this cuz why not?
    }
}


int currentQuestion = 1; //global instance
bool isSubmitted = false;

void handleInput(){

    char userOption;

    cout << "Enter your answer: \n";

        cin >> userOption;
        userOption = toupper(userOption);

        switch(userOption){
            case 'A':
            case 'B':
            case 'C':
            case 'D': 
                userAnswers[currentQuestion -1] = userOption; 
                currentQuestion++;
                break;
            case 'N': skipToNext(); break;
            case 'P': //previous
                moveToPrevious(); break;
            case 'F': submitTest(); break;

            default:
                cout << "Invalid option!\n";
        }

}

void handleQuestions(){

    do{
        cout << "Question " << currentQuestion << '\n';
        cout << questions[currentQuestion].questionContent << '\n';
        cout << "A. " << questions[currentQuestion].options[0] << '\t' << "B. " << questions[currentQuestion].options[1]  << '\n';
        cout << "C. " << questions[currentQuestion].options[2]  << '\t' << "D. " << questions[currentQuestion].options[3]  << '\n';
        
        handleInput(); //called for each question
        // currentQuestion++;

        if(currentQuestion == 10){
            submitTest();
        }
    }
    while(!isSubmitted && currentQuestion <= 10);
}

void moveToPrevious(){
    if(currentQuestion <= 1){
        cout << "No questions before Question 1\n";
    }
    else{
        currentQuestion--;
    }
}

void skipToNext(){
    if(currentQuestion >= 10){
        cout << "No questions after Question 10\n";
    }
    else{
        currentQuestion++;
    }
}

void submitTest(){
    isSubmitted = true;
    cout << "\nProcessing\n";
    //check score
    displayResults();
}

inline int checkUserAnswers(){
    int correct = 0;

    for (int i = 0; i < questions.size(); i++) {
        if (userAnswers[i] == questions[i + 1].correctOption) {
            // correct
            correct++;
        }
    }
    return correct;
}

inline bool checkSpecificQuestion(int questionNumber){
    if (userAnswers[questionNumber] == questions[questionNumber + 1].correctOption) {
        // correct
        return true;
    }
    else{
        return false;
    }
}

int displayResults(){
    cout << "You scored " << checkUserAnswers() << " out of 10\n";

    cout << "To view your script press `V`\n";

    char userChoice;
    cin >> userChoice;

    userChoice = toupper(userChoice);

    if(userChoice == 'V'){
        for (int i = 0; i < questions.size(); i++) {
            cout << "Question " << i+1 << '\n';
            cout << questions[i+1].questionContent << '\n';
            cout << "A. " << questions[i+1].options[0] << '\t' << "B. " << questions[i+1].options[1]  << '\n';
            cout << "C. " << questions[i+1].options[2]  << '\t' << "D. " << questions[i+1].options[3]  << '\n';
            cout << "Correct Option: " << questions[i+1].correctOption << '\t'
                    << "Your Choice: " << userAnswers[i] << '\t'
                    << (checkSpecificQuestion(i) ? "Correct!\n" : "Wrong!\n");
        }
    } 
    else{return 0;}
}


int main(){
    handleQuestions();
}