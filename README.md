# push-the-button

Button-pushing game

## Building

![Arduino set up with buttons and LEDs.
There are 11 sets of buttons and LEDs, each set up in the same way but connected to a different pin.
The first 10 are connected to the pins 13, 12, 11, 10, 9, 8, 7, 6, 5, 4.
The last is connected to the pin 3. This is the reset button.
The buttons go across 5V to the pin. There is an LED and a 220Ω resistor between the pin and ground.
](circuit.png).

## Simulating

Docker can be used to play the game without an Arduino.

```bash
docker container run --rm -p 8080:80 "$(docker image build -q .)"
# navigate to http://localhost:8080/
```

## Prototype

To run the prototype web browser version run:

```bash
docker container run --rm -p 8080:8080 ccouzens/buttons_game
# navigate to http://localhost:8080/
```
