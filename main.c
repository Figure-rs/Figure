typedef int Window;

Window **create_figure_window(const char* title,int width,int height);
void run_figure_window(Window **window);
#include <stdio.h>
//
//int main() {
//    create_figure_window("Hello World from C!",1280,720);
//    run_figure_window();
//    return 0;
//}

int main() {
    Window **window = create_figure_window("Hello Figure from C!",1280,720);
    printf("Created window");
    run_figure_window(window);
    return 0;
}