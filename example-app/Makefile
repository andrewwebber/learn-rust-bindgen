build:
	gcc -c -Wall -Werror -fpic add.cpp
	gcc -shared -o libexample.so add.o
	nm -gDC ./libexample.so
