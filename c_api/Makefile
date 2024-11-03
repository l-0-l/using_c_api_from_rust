CC = gcc
CFLAGS = -Wall -O2
PICFLAGS = -fPIC

SRC = api.c
OBJ = api.o
LIB_NAME = libapi

all: $(LIB_NAME).a $(LIB_NAME).so

$(OBJ): $(SRC)
	$(CC) $(CFLAGS) $(PICFLAGS) -c $(SRC) -o $(OBJ)

$(LIB_NAME).a: $(OBJ)
	ar rcs $(LIB_NAME).a $(OBJ)

$(LIB_NAME).so: $(OBJ)
	$(CC) -shared -o $(LIB_NAME).so $(OBJ)

clean:
	rm -f $(OBJ) $(LIB_NAME).a $(LIB_NAME).so
