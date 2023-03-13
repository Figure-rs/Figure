#include "figure.h"

int main() {
    Window *window = figure_create_window();
    FWidget *widget = figure_new_rectangle(Aqua,Red,20,20,100,100);
    figure_add(window,widget);
    figure_set_window_title(window,"Hello from C!");
    figure_set_window_size(window,500,500);
    figure_run_window(window);
    return 0;
}