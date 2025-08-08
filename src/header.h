#ifndef CUBE_H
#define CUBE_H

#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#define NUM_CUBIES 26
#define MAX_COLORS_PER_CUBIE 3

typedef enum color_enum {
  WHITE,
  BLUE,
  ORANGE,
  GREEN,
  RED,
  YELLOW
} Color;

typedef enum cubie_type_enum {
  CENTER=1,
  EDGE=2,
  CORNER=3
} CubieType;

typedef struct cubie_struct {
  CubieType type;
  char position;
  int rotation;

  Color colors[MAX_COLORS_PER_CUBIE];
} Cubie;

typedef struct cube_struct {
  Cubie cubies[NUM_CUBIES];
  bool solved;
} Cube;


// Debug functions
const char* color_to_string(Color c);
const char* cubie_type_to_string(CubieType t);
const void print_cubie(Cubie* c);

#endif
