/* Figure - GUI Toolkit
 *
 * MIT LICENSE
 *
 * Copyright (c) 2023 Lattexshz
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

typedef struct window {} Window;
typedef struct fwidget {} FWidget;

enum color {
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
}

Window *figure_create_window();
FWidget *figure_new_rectangle(Color color,Color border_color,unsigned int x,unsigned int y,unsigned int width,unsigned int height);
FWidget *figure_new_label(Color background_color,Color foreground_color,const char* text,unsigned int x,unsigned int y,unsigned int width,unsigned int height,unsigned int px);

void figure_set_widget_background_color(FWidget *widget,Color color);

void figure_add(Window *window,FWidget *widget);

void figure_set_window_title(Window *window,const char* title);
void figure_set_window_size(Window *window,int width,int height);

void figure_run_window(Window *window);