typedef struct window {
} Window;

Window *create_figure_window(const char* title,int width,int height);
void set_figure_window_title(Window *window,const char* title);
void run_figure_window(Window *window);
#include <stdio.h>

int main() {
    Window *window = create_figure_window("Hello World",1280,720);
    set_figure_window_title(window,"Hello World from C Lang!");
    run_figure_window(window);
    return 0;
}