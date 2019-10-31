FROM trzeci/emscripten:1.39.0 AS compiler
COPY game.c game_web.c game.h ./
RUN emcc -O3 -s WASM=1 -s EXTRA_EXPORTED_RUNTIME_METHODS='["cwrap"]' game.c game_web.c

FROM nginx:1.17.5
COPY index.html /usr/share/nginx/html/
COPY --from=compiler /src/a.out.wasm /src/a.out.js /usr/share/nginx/html/
EXPOSE 80
