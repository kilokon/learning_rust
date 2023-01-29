#include "glad/glad.h"
#include <GLFW/glfw3.h>

#include <iostream>

const unsigned int SCR_WIDTH = 800;
const unsigned int SCR_HEIGHT = 600;

// window resize function preview
void framebuffer_size_callback(GLFWwindow *window, int width, int height);

int main() {
  // first initialize GLFW with glfwInit function
  glfwInit();
  // configure GLFW using glfwWindowHint
  // https://www.glfw.org/docs/latest/window.html#window_hints
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

  // if on mac use
  //  glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
  //
  // glfwcreateWindow function takes 5 parameters
  // width: compulsory int value
  // height: compulsory int value
  // title: compulsory string
  // last two are optional for Monitor and Share: use NULL
  GLFWwindow *window =
      glfwCreateWindow(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", NULL, NULL);
  // glfwcreateWindow function returns a GLFWwindow object

  // In case window creation fails
  if (window == NULL) {
    std::cout << "Failed to create GLFW window" << std::endl;
    glfwTerminate();
    return -1;
  }

  // makes GLFW window the current context
  glfwMakeContextCurrent(window);

  // Handles window resize closure
  glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);

  // Since GLAD manages function pointers
  // test to check if GLAD connects properly
  if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
    std::cout << "Failed to initialize GLAD" << std::endl;
    return -1;
  }

  // This tell the program to keep drawing images and handling user
  // input until the program has been explicitly told to stop.
  while (!glfwWindowShouldClose(window)) {
    glfwSwapBuffers(window);
    glfwPollEvents();
  }

  // glfwTerminate cleans up after closing the GLFW window
  glfwTerminate();
  return 0;
}

// Window resize event handler
void framebuffer_size_callback(GLFWwindow *window, int width, int height) {
  glViewport(0, 0, width, height);
}
