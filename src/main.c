#include "header.h"

int main() {

  Cubie Cubie = {
    CORNER,
    'a',
    0,
    {WHITE, ORANGE, BLUE}
  };

  print_cubie(&Cubie);

  return 1;
};
