
#include <stdio.h>

typedef struct a_string {
  char* name;
} MyType;

extern MyType MY_STRING;

void main() {
  printf("name: %s\n", MY_STRING.name);
}
