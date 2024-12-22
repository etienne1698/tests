# mini-compiler
Ce projet sert les objectifs suivants:
- M'améliorer en c++
- Comprendre les base d'un compilateur 
- Aquérir des base en ASM et plus généralement dans la programmation bas niveau
- (plus tard) Apprendre à integrer LLVM à ma chaine de compilation et permettre l'appel de fonction C depuis mon langage

## Build with Cmake
```
cmake -S . -B build 
cmake --build build
```

## Run 
```
.\build\Debug\compile tests/simple.cm
```

## "cm" langage description and examples

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