out : main.o matrix.o 
	cc -o out main.o matrix.o 

main.o : main.c  
	cc -c main.c
matrix.o : matrix.c matrix.h
	cc -c matrix.c
clean :
	rm out main.o matrix.o