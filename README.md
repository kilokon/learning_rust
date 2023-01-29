### Checking OpenGL version check
Arch Linux 64bit, Nvidia Proprietary drivers.
```glxewinfo | grep OpenGL```

### Install Components
```sudo pacman -S glfw-x11```

### Include glad Library
1. Glad header generated from [http://glad.dav1d.de]
2. Get approprate C++ & OpenGl & Version and core ```glad.zip``` file.
2. Copy all in include directory to the system include ```cp -R include/* /usr/include/```
3. Copy glad.c in ```src/``` to ```program.cpp```
4. run with ```g++ program.cpp glad.c -ldl -lglfw```
