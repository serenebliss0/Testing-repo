#this language is quite cooked

name <- "Semire"
age <- 16
height <- 200.18
is_student <- TRUE

name
age

for (x in 1:10) {
  print(x)
}


text <- "awesome"
paste("R is", text)
text2 <- "R is not"
facts <- paste("", text2, "awesome")

print(facts)

num1 <- 5
num2 <- 10
num1 + num2

var1 <- var2 <- var3 <- var4 <- "orange"

var3

s <- 1000L
#this declares a var "s" as an integer
class(s)
typeof(s)

l <- 9i + 3
class(l)

d <- 3.14
#this declares a var "d" as a numeric (why would they call it that)
#double or float sounds better

typeof(d)
class(d)

x <- 1L
y <- 2

a <- as.numeric(x)
b <- as.integer(y)

k <- 2 + 5i
g <- as.integer(k)

print(k)
print(g)


summary("students_scores.csv")

# now lets do some simple math

10 + 5

10 - 5

max(5, 10, 15, 20)
min(5, 10, 15, 20)

sqrt(16)

abs(-50)

ceiling(5.7)
floor(5.3)

#Stringsssss

"Hello, World!"

str <- "Hello, World!"
print(str)


nchar(str)

is_present <- grepl("H", str)
#grep?!?!?! What are you doing here?
# The grepl function is used to search for patterns in strings. 
# It returns TRUE if the pattern is found and FALSE otherwise.

if (is_present == TRUE) {
  print("The letter 'H' is present in the string.")
}

tail('students_scores.csv')
data <- read.csv("students_scores.csv")