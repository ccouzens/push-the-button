# push-the-button

Button-pushing game

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
