typedef struct window {} Window;
typedef struct fwidget {} FWidget;

typedef enum {
    White,
    Olive,
    Yellow,
    Fuchsia,
    Silver,
    Aqua,
    Lime,
    Red,
    Gray,
    Blue,
    Green,
    Purple,
    Black,
    Navy,
    Teal,
    Maroon
} Color;

Window *figure_create_window();
FWidget *figure_new_rectangle(Color color,Color border_color,unsigned int x,unsigned int y,unsigned int width,unsigned int height);

void figure_add(Window *window,FWidget *widget);

void figure_set_window_title(Window *window,const char* title);
void figure_set_window_size(Window *window,int width,int height);

void figure_run_window(Window *window);