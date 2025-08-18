#ifndef CUBE_H
#define CUBE_H

#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "cube_data.h"

// Utilitary functions
void print_cube(const Cube* c);
Cube* generate_cube();
Cube* scramble_cube(Cube* cube);

#endif
