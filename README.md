### Checking OpenGL version system
Running Arch Linux 64bit, Nvidia Proprietary drivers.

### Install Components
```
glxewinfo | grep OpenGL
sudo pacman -S glfw-x11
```
### Include glad Library
1. Glad header generated from [http://glad.dav1d.de]
2. Copy all in include directory to the system include ```cp -R include/* /usr/include/```
3. Copy glad.c in ```src/``` to ```program.cpp```
4. run with ```g++ program.cpp glad.c -ldl -lglfw```
