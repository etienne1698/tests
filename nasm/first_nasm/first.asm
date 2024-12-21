section .data
    message db "Hello, World!", 0Ah  ; Message avec saut de ligne
    msg_len equ $ - message          ; Longueur du message

section .text
    global main                      ; Point d'entrée

    extern _GetStdHandle@4           ; Symboles externes avec décorations
    extern _WriteConsoleA@20
    extern _ExitProcess@4

main:
    ; Obtenir le handle de la console standard
    push -11                         ; STD_OUTPUT_HANDLE (-11)
    call _GetStdHandle@4
    mov ebx, eax                     ; Stocker le handle dans EBX

    ; Écrire le message sur la console
    push 0                           ; NULL (Pas d'overlapped structure)
    push dword 0                     ; NULL (Pas de bytes_written)
    push dword msg_len               ; Taille du message
    push message                     ; Adresse du message
    push dword ebx                   ; Handle de la console
    call _WriteConsoleA@20

    ; Quitter le programme
    push 0                           ; Code de retour
    call _ExitProcess@4
