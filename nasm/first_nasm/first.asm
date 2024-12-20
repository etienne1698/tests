section .data
    message db "Hello, World!", 0Ah  ; Message avec un saut de ligne
    msg_len equ $ - message          ; Longueur du message

section .bss
    hConsole resd 1                  ; Handle de la console

section .text
    global main                      ; Point d'entrée du programme

extern GetStdHandle                 ; Fonction pour obtenir le handle de la console
extern WriteConsoleA                ; Fonction pour écrire sur la console
extern ExitProcess                  ; Fonction pour quitter le programme

main:
    ; Obtenir le handle de la sortie standard (STD_OUTPUT_HANDLE)
    push -11                         ; STD_OUTPUT_HANDLE
    call GetStdHandle
    mov [hConsole], eax              ; Stocker le handle

    ; Écrire le message sur la console
    push 0                           ; Pas de structure d'écriture (NULL)
    push dword 0                     ; Pas de retour d'octets écrits (NULL)
    push dword msg_len               ; Taille du message
    push message                     ; Adresse du message
    push dword [hConsole]            ; Handle de la console
    call WriteConsoleA

    ; Quitter le programme
    push 0                           ; Code de sortie
    call ExitProcess