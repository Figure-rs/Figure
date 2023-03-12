void create_figure_window(const char* title,int width,int height);
void run_figure_window();
#include <stdio.h>

int main() {
    create_figure_window("Hello World from C!",1280,720);
    run_figure_window();
    return 0;
}