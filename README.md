# Readrix

New Tab article reader extension built with Yew.

## Run

1. Styles:

```
npm i -g tailwindcss

tailwindcss -i index.css -o ./tailwind.css --watch
```

2. App

```
trunk serve
```


## Build

1. Styles:

```
NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
```

2. App

```
trunk build --release && cp ./manifest.json ./write_wasm.js ./dist
```