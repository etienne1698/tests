section .data
    input_buffer db 128                 ; Tampon pour stocker l'entrée utilisateur
    input_max_length equ 127            ; Longueur maximale (moins 1 pour le 0 final)
    prompt db "Entrez un texte : ", 0   ; Texte d'invite
    prompt_len equ $ - prompt           ; Longueur du texte d'invite

section .bss
    bytes_read resd 1                   ; Nombre de bytes lus

section .text
    global main                         ; Point d'entrée

    extern _GetStdHandle@4              ; Obtenir les handles standards
    extern _ReadConsoleA@20             ; Lire depuis la console
    extern _WriteConsoleA@20            ; Écrire dans la console
    extern _ExitProcess@4               ; Terminer le programme

main:
    ; Obtenir le handle pour la sortie standard
    push -11                            ; STD_OUTPUT_HANDLE (-11)
    call _GetStdHandle@4
    mov ebx, eax                        ; Stocker le handle de sortie dans EBX

    ; Afficher le texte d'invite
    push 0                              ; NULL (Pas d'overlapped structure)
    push 0                              ; NULL (Pas de bytes_written)
    push prompt_len                     ; Longueur du message d'invite
    push prompt                         ; Adresse du message d'invite
    push ebx                            ; Handle de la console
    call _WriteConsoleA@20

    ; Obtenir le handle pour l'entrée standard
    push -10                            ; STD_INPUT_HANDLE (-10)
    call _GetStdHandle@4
    mov ecx, eax                        ; Stocker le handle d'entrée dans ECX

    ; Lire l'entrée utilisateur
    push 0                              ; NULL (Pas d'overlapped structure)
    push bytes_read                     ; Adresse pour stocker bytes_read
    push input_max_length               ; Taille maximale du tampon
    push input_buffer                   ; Adresse du tampon
    push ecx                            ; Handle de la console d'entrée
    call _ReadConsoleA@20

    ; Écrire l'entrée utilisateur sur la console
    push 0                              ; NULL (Pas d'overlapped structure)
    push 0                              ; NULL (Pas de bytes_written)
    push dword [bytes_read]             ; Taille lue (à écrire)
    push input_buffer                   ; Adresse du tampon à écrire
    push ebx                            ; Handle de la console de sortie
    call _WriteConsoleA@20

    ; Quitter le programme
    push 0                              ; Code de retour
    call _ExitProcess@4
