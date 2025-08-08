#include "header.h"

const char* color_to_string(Color c) {
  switch (c) {
    case WHITE:  return "White";
    case BLUE:   return "Blue";
    case ORANGE: return "Orange";
    case GREEN:  return "Green";
    case RED:    return "Red";
    case YELLOW: return "Yellow";
    default:     return "Unknown Color";
  }
}

const char* cubie_type_to_string(CubieType t) {
  switch (t) {
    case CENTER: return "Center";
    case EDGE:   return "Edge";
    case CORNER: return "Corner";
    default:     return "Unknown Type";
  }
}

void print_cubie(Cubie* c) {
  printf("Cubie Type: %s\n", cubie_type_to_string(c->type));
  printf("Position: %c\n", c->position);
  printf("Rotation: %d\n", c->rotation);
  printf("Colors: ");
  for (int i = 0; i < c->type; i++) { // type determines number of colors
    printf("%s ", color_to_string(c->colors[i]));
  }
  printf("\n");
}

