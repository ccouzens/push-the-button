#include <stdint.h>;

#define PUSH_THE_BUTTON_SIZE 10

struct push_the_button_game {
  uint8_t leds[PUSH_THE_BUTTON_SIZE];
};

struct push_the_button_game push_the_button_new();

void push_the_button(struct push_the_button_game *game, uint8_t button);

void push_the_button_reset(struct push_the_button_game *game);