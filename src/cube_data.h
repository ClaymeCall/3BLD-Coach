#ifndef CUBE_DATA_H
#define CUBE_DATA_H

#include <stdlib.h>
#include <stdio.h>

typedef enum {
  WHITE,
  GREEN,
  RED,
  BLUE,
  ORANGE,
  YELLOW,
} Face;

typedef struct {
  int id;
  int position;
  int orientation;
} CornerCubie;

typedef struct {
  int id;
  int position;
  int orientation;
} EdgeCubie;

typedef struct cube_struct {
  CornerCubie* corners[8];
  EdgeCubie* edges[12];
} Cube;

typedef struct {
  Face face;
  int row, col;
} StickerCoord;

// ---------------- CORNER COORDINATES ----------------
// For each *position* (0–7), we define where its 3 stickers appear
// Coordinates correspond to CORNER_PIECES[position]
extern const StickerCoord cornerStickerCoords[8][3];

// ---------------- EDGE COORDINATES ----------------
extern const StickerCoord edgeStickerCoords[12][2];

/*
 * face_to_ansi - Returns colored squares using ANSI codes depending on Face color
 * @f: Face color to translate to ANSI square
 *
 * Return: ANSI square of corresponding color
 */
static const char* face_to_ansi(Face f) {
  switch (f) {
    case WHITE:  return "\033[48;2;255;255;255m  \033[0m";
    case GREEN:  return "\033[48;2;0;255;50m  \033[0m";
    case RED:    return "\033[48;2;235;0;0m  \033[0m";
    case BLUE:   return "\033[48;2;50;120;255m  \033[0m";
    case ORANGE: return "\033[48;2;255;125;50m  \033[0m";
    case YELLOW: return "\033[48;2;235;235;50m  \033[0m";
    default:     return "  ";
  }
}

/*
 * char_to_face - Returns a Face color depending on face Char name
 * @c: Char to translate to Face color
 *
 * Return: Face color corresponding to char
 */
static Face char_to_face(char c) {
    switch (c) {
        case 'U': return WHITE;
        case 'F': return GREEN;
        case 'R': return RED;
        case 'B': return BLUE;
        case 'L': return ORANGE;
        case 'D': return YELLOW;
        default:  return WHITE; // fallback
    }
}

extern const char CORNER_PIECES[8][3];
extern const char EDGE_PIECES[12][2];

int move_R(Cube* cube);

#endif
