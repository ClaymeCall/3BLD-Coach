#include "cube_data.h"

const char CORNER_PIECES[8][3] = {
  {'U','L','B'},
  {'U','B','R'},
  {'U','R','F'},
  {'U','F','L'},
  {'D','L','F'},
  {'D','F','R'},
  {'D','R','B'},
  {'D','B','L'},
};

// ---------------- CORNER COORDINATES ----------------
// For each *position* (0–7), we define where its 3 stickers appear
// Coordinates correspond to CORNER_PIECES[position]
const StickerCoord cornerStickerCoords[8][3] = {
  // ULB (0)
  { {WHITE,0,0},
    {ORANGE,0,2},
    {BLUE,0,0} },
  // UBR (1)
  { {WHITE,0,2},
    {BLUE,0,2},
    {RED,0,0} },
  // URF (2)
  { {WHITE,2,2},
    {RED,0,2},
    {GREEN,0,0} },
  // UFL (3)
  { {WHITE,2,0},
    {GREEN,0,2},
    {ORANGE,0,0} },
  // DLF (4)
  { {YELLOW,0,0},
    {ORANGE,2,0},
    {GREEN,2,2} },
  // DRF (5)
  { {YELLOW,0,2},
    {GREEN,2,0},
    {RED,2,2} },
  // DRB (6)
  { {YELLOW,2,2},
    {RED,2,0},
    {BLUE,2,2} },
  // DBL (7)
  { {YELLOW,2,0},
    {BLUE,2,0},
    {ORANGE,2,2} },
};

const char EDGE_PIECES[12][2] = {
  {'U','B'},
  {'U','R'},
  {'U','F'},
  {'U','L'},
  {'D','F'},
  {'D','R'},
  {'D','B'},
  {'D','L'},
  {'F','L'},
  {'F','R'},
  {'B','R'},
  {'B','L'},
};

// ---------------- EDGE COORDINATES ----------------
const StickerCoord edgeStickerCoords[12][2] = {
  // UB (0)
  {{WHITE,0,1}, {BLUE,0,1}},
  // UR (1)
  {{WHITE,1,2}, {RED,0,1}},
  // UF (2)
  {{WHITE,2,1}, {GREEN,0,1}},
  // UL (3)
  {{WHITE,1,0}, {ORANGE,0,1}},
  // DF (4)
  {{YELLOW,0,1}, {GREEN,2,1}},
  // DR (5)
  {{YELLOW,1,2}, {RED,2,1}},
  // DB (6)
  {{YELLOW,2,1}, {BLUE,2,1}},
  // DL (7)
  {{YELLOW,1,0}, {ORANGE,2,1}},
  // FL (8)
  {{GREEN,1,0}, {ORANGE,1,2}},
  // FR (9)
  {{GREEN,1,2}, {RED,1,0}},
  // BR (10)
  {{BLUE,1,2}, {RED,1,2}},
  // BL (11)
  {{BLUE,1,0}, {ORANGE,1,0}},
};

int move_R(Cube* cube) {
  if(cube == NULL)
    return (-1);

  return 1;
}
