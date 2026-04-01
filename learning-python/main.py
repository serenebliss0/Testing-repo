"""
Text Type:	str
Numeric Types:	int, float, complex
Sequence Types:	list, tuple, range
Mapping Type:	dict
Set Types:	set, frozenset
Boolean Type:	bool
Binary Types:	bytes, bytearray, memoryview
None Type:	NoneType
"""
x = 5
print(type(x))

memoryview(bytes(5))

import random

print(random.randrange(1,100));

a = """
Lorem ipsum dolor sit amet,
consectetur adipiscing elit,
sed do eiusmod tempor incididunt
ut labore et dolore magna aliqua.

"""

print(a);

for x in "semire":
    print(x)
    print(len(a))

txt = "Semire is the goat"; print(txt);
if not "Semire" in txt:
    print("This person is not the goat");

lies = "This is a lie"
print("Not a lie" not in lies) # Returns true!

# lets slice some arrays
### remember when the upper bound wont be included!!!
more_text = "Hello world";
print(more_text[2:5]);
# just like rust, it prints from the 2nd char to the 4th char
# just use `:` instead of `..`

# if you ign the lower bound it just prints from the start (hehh)
print(more_text[:5]);

#same goes for leaving the upper bound
print(more_text[2:]);

#python has something a bit weirder though... negative indexing

print(more_text[-5:-2]) ### prints "wor"
# basically it starts counting from the end of the string instead
#hello world -> !dlorw olleh

#remember array.chars(),rev(),collect<String>(); in rust?
# you can do that very easily:

array = [1, 2, 3, 4]
print(array[::-1]) #just like that :sob:

###something very funny will happen if you make it a single : instead of ::

## the syntax is array[start: stop: step]

array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
print(array[0: len(array) -1: 2]); # this is basically array[i+2]
# it increments the idx by two on every pass

truth = "seMire is stIll, the GOaT "

print(truth.upper());

print(truth.lower());

#dont forget our good ol', .trim()
print(truth.strip())

print(truth.replace("GOaT", "best"));

print(truth.split(","))

pi = 3.141592
print(f"PI is equal to {pi:.2f}"); #rounds to 2dp


#remember verbatim literals from C#?
#python has its own escape chars too!

txt = "Semire is \"still\"the goat";

#python bools are a little weird for me

print(10 > 9)

print(bool("Hello")) #somehow this returns true :sob"
#basically any number, except 0 will return True
#any string, except its empty will return True as well

#only a few values, like 0s, unit type (if its called that here), empty DSs
#will return False

bool(False)

print(bool("")) #Returns false

# you can check if something is a certain data type too
x = 200.0
print(isinstance(x, int))

#some operators speak for themselves too well

x = [1, 2, 4, 5, 6]
y = [1, 2, 4, 5, 6]



print("Yes", x is not y) #funny enough this will return True cuz 
#x and y dont point to the same object!!!

x = y = [1, 2, 3, 4]
print("Uhhh", x is not y) #returns false now cuz they both point to the same list

#essentially, is checks if the vars point to the same obj in memory!!!
# = only compares values!!!

def my_function(list):
    print("List immediately after pass");
    list.append([1,2]);
    print("List inside function: ", list);

list = [1,2,3,4];
my_function(list);
print("List outside function", list);

print(bool(""))

x = 5

print(x);

x = 10

print(x);

x = 10

def change():
    global x
    x = 20


change()

print(x)