import { Game } from "crystal-maze";
import { memory } from "crystal-maze/crystal_maze_bg";

const ACTIONS = [
  "All off",
  "All but self toggle",
  "Toggle self",
  "On evens",
  "Off evens",
  "On odds",
  "Off odds",
  "Toggle ±1",
  "Toggle ±2",
  "Toggle ±3"
];

const game = Game.new(BigInt(new Date().getTime()));
const SIZE = game.size();

const buttonElements = document
  .getElementById("leds")
  .getElementsByTagName("button");

const ledsPtr = game.leds();
const leds = new Uint8Array(memory.buffer, ledsPtr, SIZE);
const mappingPtr = game.mapping();
const mapping = new Uint8Array(memory.buffer, mappingPtr, SIZE);

const setLeds = () =>
  leds.forEach((on, i) => {
    buttonElements[i].classList.toggle("on", on !== 0);
  });

const displayMapping = () => {
  let displayedMapping = "";
  const a = "a".codePointAt(0);
  for (let i = 0; i < SIZE; i++) {
    const mapped = mapping[i];
    displayedMapping += `${i + 1}: ${String.fromCodePoint(a + mapped)} (${
      ACTIONS[mapped]
    })\n`;
  }
  document.getElementById("mapping").textContent = displayedMapping;
};

for (let i = 0; i < SIZE; i++) {
  const buttonElement = buttonElements[i];
  buttonElement.addEventListener("click", () => {
    game.button_down(i);
    setLeds();
  });
}

document.getElementById("reset").addEventListener("click", () => {
  game.reset();
  setLeds();
  displayMapping();
});

setLeds();
displayMapping();
