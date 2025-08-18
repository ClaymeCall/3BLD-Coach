#include "header.h"

#include <stdlib.h>

int main() {

  Cube* cube = generate_cube();

  if (cube == NULL)
    printf("Failed cube generation\n");

  print_cube(cube);

  scramble_cube(cube);
    printf("Scrambled cube\n");

  print_cube(cube);

  free(cube);

  return 1;
};
