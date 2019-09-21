import { Game } from "crystal-maze";
import { memory } from "crystal-maze/crystal_maze_bg";

const game = Game.new();
const SIZE = game.size();

const buttonElements = document
  .getElementById("leds")
  .getElementsByTagName("button");

const ledsPtr = game.leds();
const leds = new Uint8Array(memory.buffer, ledsPtr, SIZE);

const setLeds = () =>
  leds.forEach((on, i) => {
    buttonElements[i].classList.toggle("on", on !== 0);
  });

for (let i = 0; i < SIZE; i++) {
  const buttonElement = buttonElements[i];
  buttonElement.addEventListener("mousedown", () => {
    game.button_down(i);
    setLeds();
  });
}

setLeds();
