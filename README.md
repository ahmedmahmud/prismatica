<div align="center">

```ocaml
Prismatica
```
<!-- ![Preview](assets/output.png) -->

<samp>Convert image colors to palettes</samp>

---
<div align="left">

> 🚧 ~ This project is in development

## <samp> Building </samp>
Clone repo
```console
git clone https://github.com/ahmedmahmud/prismatica.git
cd prismatica
```

Build the tauri app
```console
npm run tauri build
```
The output directory for the binary is
```
./src-tauri/target/release
```

## <samp> Usage </samp>
There are two ways to use this app:
- GUI
- CLI

### GUI
> *🚧 Under construction*

### CLI
Run the CLI with the path to the input image, the noise amount and the desired palette
```console
./prismatica path/to/image.png -n 2 -p mocha
```
This will create a file `mocha-image.png` for your converted image