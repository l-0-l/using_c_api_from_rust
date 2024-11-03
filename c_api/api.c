#include <stdio.h>

int add_numbers(int a, int b) {
    return a + b;
}

void greet() {
    printf("Hello from C!\n");
}

// Don't forget to `export LD_LIBRARY_PATH=/whatever/your/path/is/c_api/`
