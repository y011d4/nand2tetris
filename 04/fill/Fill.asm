// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.
(LOOP)
    @KBD
    D=M
    @BLACK
    D;JNE
    @WHITE
    0;JMP
(LOOPEND)
    @LOOP
    0;JMP

(BLACK)
    @b_or_w
    M=-1
    @FILL
    0;JMP
(WHITE)
    @b_or_w
    M=0
    @FILL
    0;JMP

(FILL)
    @i
    M=0
(FILLLOOP)
    @i
    D=M
    @8192
    D=D-A
    @LOOPEND
    D;JGE
    @SCREEN
    D=A
    @i
    D=D+M
    @address
    M=D
    @b_or_w
    D=M
    @address
    A=M
    M=D
    @i
    M=M+1
    @FILLLOOP
    0;JMP
