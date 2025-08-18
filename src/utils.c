#include "header.h"

/*
 * print_cube - Prints a flat pattern view of a cube with ANSI colors
 * @Cube: Pointer to the cube to print
 *
 * Return: void
 */
void print_cube(const Cube *c) {
  char stickers[6][3][3];

  // init to spaces
  for (int f = 0; f < 6; f++)
    for (int r = 0; r < 3; r++)
      for (int col = 0; col < 3; col++)
        stickers[f][r][col] = ' ';

  // ----- Place corners -----
  for (int i = 0; i < 8; i++) {
    CornerCubie *cubie = c->corners[i];
    int id = cubie->id;
    int pos = cubie->position;
    int ori = cubie->orientation;

    char colors[3];
    for (int j = 0; j < 3; j++)
      colors[j] = CORNER_PIECES[id][j];

    for (int j = 0; j < 3; j++) {
      StickerCoord sc = cornerStickerCoords[pos][j];
      char colChar = colors[(j + ori) % 3];
      stickers[sc.face][sc.row][sc.col] = colChar;
    }
  }

  // ----- Place edges -----
  for (int i = 0; i < 12; i++) {
    EdgeCubie *cubie = c->edges[i];
    int id = cubie->id;
    int pos = cubie->position;
    int ori = cubie->orientation;

    char colors[2];
    for (int j = 0; j < 2; j++)
      colors[j] = EDGE_PIECES[id][j];

    for (int j = 0; j < 2; j++) {
      StickerCoord sc = edgeStickerCoords[pos][j];
      char colChar = colors[(j + ori) % 2];
      stickers[sc.face][sc.row][sc.col] = colChar;
    }
  }

  // ----- Centers -----
  stickers[WHITE][1][1]  = 'U';
  stickers[GREEN][1][1]  = 'F';
  stickers[RED][1][1]    = 'R';
  stickers[BLUE][1][1]   = 'B';
  stickers[ORANGE][1][1] = 'L';
  stickers[YELLOW][1][1] = 'D';

  // ----- Print in cube net format -----
  printf("\n");

  // U
  for (int r = 0; r < 3; r++) {
    printf("        "); // Offset by 1 face width + 1 spacing
    for (int c2 = 0; c2 < 3; c2++) {
      Face f = char_to_face(stickers[WHITE][r][c2]);
      printf("%s", face_to_ansi(f));
    }
    printf("\n");
  }

  printf("\n");

  // L F R B
  for (int r = 0; r < 3; r++) {
    for (int c2 = 0; c2 < 3; c2++) {
      Face f = char_to_face(stickers[ORANGE][r][c2]);
      printf("%s", face_to_ansi(f));
    }
    printf("  ");
    for (int c2 = 0; c2 < 3; c2++) {
      Face f = char_to_face(stickers[GREEN][r][c2]);
      printf("%s", face_to_ansi(f));
    }
    printf("  ");
    for (int c2 = 0; c2 < 3; c2++) {
      Face f = char_to_face(stickers[RED][r][c2]);
      printf("%s", face_to_ansi(f));
    }
    printf("  ");
    for (int c2 = 0; c2 < 3; c2++) {
      Face f = char_to_face(stickers[BLUE][r][c2]);
      printf("%s", face_to_ansi(f));
    }
    printf("\n");
  }

  printf("\n");

  // D
  for (int r = 0; r < 3; r++) {
    printf("        "); // Offset by 1 face width + 1 spacing
    for (int c2 = 0; c2 < 3; c2++) {
      Face f = char_to_face(stickers[YELLOW][r][c2]);
      printf("%s", face_to_ansi(f));
    }
    printf("\n");
  }

  printf("\n");
}

/*
 *generate_cube - Generates a fully constructed Cube object and all its Cubies
 *in solved state
 *
 *Return: Pointer to the cube
 */
Cube* generate_cube() {
  Cube *cube = malloc(sizeof(Cube));
  int i = 0;

  // Adding Corner cubies to cube
  for (i = 0; i < 8; ++i) {
    CornerCubie *cubie = malloc(sizeof(CornerCubie));
    if (cubie == NULL)
      return (NULL);

    cubie->id = cubie->position = i;
    cubie->orientation = 0;

    cube->corners[i] = cubie;
  }

  // Adding Edge cubies to cube
  for (i = 0; i < 12; ++i) {
    EdgeCubie *cubie = malloc(sizeof(EdgeCubie));
    if (cubie == NULL)
      return (NULL);

    cubie->id = cubie->position = i;
    cubie->orientation = 0;

    cube->edges[i] = cubie;
  }

  return cube;
};

// Scramble a cube
Cube* scramble_cube(Cube* cube) {
  // Seed random
  unsigned int seed = time(NULL);

  srand(seed);

  // --- Shuffle corners ---
  for (int i = 7; i > 0; i--) {
    int j = rand() % (i + 1);
    // Swap positions
    int temp = cube->corners[i]->position;
    cube->corners[i]->position = cube->corners[j]->position;
    cube->corners[j]->position = temp;
    // Random orientation 0,1,2
    cube->corners[i]->orientation = rand() % 3;
  }

  // Ensure total corner orientation modulo 3 == 0 (legal cube)
  int corner_ori_sum = 0;
  for (int i = 0; i < 8; i++) corner_ori_sum += cube->corners[i]->orientation;
  cube->corners[7]->orientation = (3 - (corner_ori_sum % 3)) % 3;

  // --- Shuffle edges ---
  for (int i = 11; i > 0; i--) {
    int j = rand() % (i + 1);
    int temp = cube->edges[i]->position;
    cube->edges[i]->position = cube->edges[j]->position;
    cube->edges[j]->position = temp;
    // Random orientation 0,1
    cube->edges[i]->orientation = rand() % 2;
  }

  //Ensure total edge orientation modulo 2 == 0 (legal cube)
  int edge_ori_sum = 0;
  for (int i = 0; i < 12; i++) edge_ori_sum += cube->edges[i]->orientation;
  cube->edges[11]->orientation = (2 - (edge_ori_sum % 2)) % 2;

  return cube;
}
