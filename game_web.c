#include "game.h"
#include <emscripten.h>
#include <stdlib.h>

long push_the_button_random(long max) {
  double jsRandom = EM_ASM_DOUBLE({ return Math.random(); });
  return (long)(jsRandom * max);
}

EMSCRIPTEN_KEEPALIVE
struct push_the_button_game *new () {
  struct push_the_button_game *game =
      malloc(sizeof(struct push_the_button_game));
  *game = push_the_button_new(&push_the_button_random);
  return game;
}

EMSCRIPTEN_KEEPALIVE
void free_game(struct push_the_button_game *game) { free(game); }

EMSCRIPTEN_KEEPALIVE
uint8_t *leds(struct push_the_button_game *game) { return game->leds; }

EMSCRIPTEN_KEEPALIVE
uint8_t *mappings(struct push_the_button_game *game) { return game->mappings; }

EMSCRIPTEN_KEEPALIVE
void push(struct push_the_button_game *game, uint8_t button) {
  push_the_button(game, button);
}

EMSCRIPTEN_KEEPALIVE
void reset(struct push_the_button_game *game) { push_the_button_reset(game); }
