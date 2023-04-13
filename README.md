<div align="center">

```ocaml
Prismatica
```
![Preview](assets/preview.png)

<samp>Convert an image's colors to different palette</samp>

---
<div align="left">

> 🚧 ~ This project is in early stages

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
Run the application
```console
./prismatica
```
Choose a file you want to upload, pick the palette and press `Generate`
From there you can pick what palette of the theme you want, in each tab all the listed images are of different noise levels. Choose which you find best and press download, the image will then automatically be downloaded to the directory you ran the application. Press view to full screen the image to get a better look

### CLI
Run the CLI with the path to the input image, the noise amount and the desired palette
```console
./prismatica path/to/image.png -n 2 -p mocha
```
This will create a file `mocha-image.png` for your converted image

## Issues
- Downloading will put file in current dir, if run through a launcher this may be awkward.   
*Show directory picker?*
- Switching between tabs quickly causes too many requests to backend and may crash your app.  
*No way to cancel promises, may switch multithreading completely to backend*
- Possible memory leak?  
*Seen high memory usage from normal usage, btop reported 1G however closing resulted in more freed. Not sure where from*
- Switching tabs doesn't cache previous images, will re-generate  
*Cache these on either frontend or backend*

## Todo
- Fix issues
- Add more palettes
- Multi file selection/Batch jobs