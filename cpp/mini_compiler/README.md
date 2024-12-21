# Build with Cmake
```
cmake -S . -B build 
cmake --build build
```

# Run 
```
.\build\Debug\compile
```

# "cm" langage description and examples

- function declaration:
```
def maFonction(arg1, arg2) {
    return arg1 + arg2
}
```
- variable declaration:
```
var maVariable = 1
```
- if statement
```
if a > 2:
    a = 1 
elseif a > 1:
    a = 2
else:
    a = 3
```
- basic functions
```
var a
print "Entrez une chaine de caractères:"
read a
print "Vous avez saisie : " + a
```