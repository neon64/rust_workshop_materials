all: data-race dangling-pointer 

clean:
	rm data-race dangling-pointer

data-race:
	gcc -Wall -o data-race data-race.c -lpthread

dangling-pointer:
	gcc -Wall -o dangling-pointer dangling-pointer.c

