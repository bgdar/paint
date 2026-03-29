#include <SDL2/SDL.h>
#include <SDL2/SDL_events.h>
#include <SDL2/SDL_mouse.h>
#include <SDL2/SDL_rect.h>
#include <SDL2/SDL_stdinc.h>
#include <SDL2/SDL_surface.h>
#include <SDL2/SDL_timer.h>
#include <SDL2/SDL_video.h>

#include <math.h>
#include <stdbool.h>

#define START_BRUSH_SIZE 10
#define COLOR_RECT_SIZE 30
#define START_COLOR 0xff0000

// Global Var
Uint32 color = START_COLOR;
Uint32 color_palette[] = {
    0x000000, 0xffffff, 0x00ff00, 0xfff000,
    0xf00f00, 0x000fff, 0xf00ff0}; // simpan data hex color
const int color_pallete_size = sizeof(color_palette);

// cek apakah di color pallet atau bukan
bool inside_color_palette(int x, int y) {
  if (x < color_pallete_size * COLOR_RECT_SIZE && y <= COLOR_RECT_SIZE) {
    return true;
  }
  return false;
}

// update  jenis warna berdasarkan pallet yang di click user , selain itu
// update warna saat ini
void check_color_palette_chosen(int x, int y) {
  int i;
  if (inside_color_palette(x, y)) {
    // mouse masuk ke x , y range color pallet
    i = x / COLOR_RECT_SIZE;
    color = color_palette[i];
  }
}

// untuk render pallet warna yang terdiri dari ukuran warna element
void drawPalette(SDL_Surface *surface, Uint32 *colors, int size) {

  SDL_Rect color_rect;
  for (int i = 0; i < size; i++) {
    color_rect = (struct SDL_Rect){i * COLOR_RECT_SIZE, 0, COLOR_RECT_SIZE,
                                   COLOR_RECT_SIZE};
    SDL_FillRect(surface, &color_rect, colors[i]);
  }
}

// gambar dari kotak -> circle
// gambar circle di tengah yang terdiri dari brush_size dan warna
void drawSurface(SDL_Surface *surface, int x_center, int y_center,
                 int brush_size, Uint32 color) {

  SDL_Rect pixel = {0, 0, 1, 1};
  for (int x = x_center - brush_size; x < x_center + brush_size; x++) {
    for (int y = y_center - brush_size; y < y_center + brush_size; y++) {

      // cek apakah pixel part adalah circel
      int distance_from_center =
          sqrt(pow(x - x_center, 2) + pow(y - y_center, 2));
      if (distance_from_center < brush_size) {

        pixel.x = x;
        pixel.y = y;

        SDL_FillRect(surface, &pixel, color);
      }
    }
  }
}

int main() {

  bool done = false;
  bool draw = false;

  int width = 600;
  int heigth = 650;

  float Target_fps = 60;
  float delai_milis = (1.0 / Target_fps) * 1000;

  int x, y;
  int brush_size = START_BRUSH_SIZE;

  SDL_Init(SDL_INIT_VIDEO);

  SDL_Window *window =
      SDL_CreateWindow("Paint", width / 2, heigth / 2, width, heigth, 0);

  SDL_Surface *surface = SDL_GetWindowSurface(window);

  drawPalette(surface, color_palette,
              color_pallete_size); // tampilkan pallet duluan di atas
  SDL_UpdateWindowSurface(window);

  while (!done) {
    SDL_Event event;

    while (SDL_PollEvent(&event)) {
      switch (event.type) {
      case SDL_QUIT:
        done = true;
        break;
        // untuk handle pergerakan mouse
      case SDL_MOUSEMOTION:
        x = event.motion.x;
        y = event.motion.y;
        break;
      // cek untuk mouse yang di click kiri baru draw di lakukan
      case SDL_MOUSEBUTTONDOWN:
        x = event.motion.x;
        y = event.motion.y;

        // cek color yang di click atau di pilih di pallet
        check_color_palette_chosen(x, y);
        if (inside_color_palette(x, y) == false)
          draw = true;

        break;
      case SDL_MOUSEBUTTONUP:
        draw = false;
        x = event.motion.x;
        y = event.motion.y;
        break;

      // ejust mouse yang draw nya , besar dan kecil ukuran brush nya
      case SDL_MOUSEWHEEL:
        // Uint32 direction = event.wheel.direction;

        brush_size += event.wheel.preciseY;
        if (brush_size < 1) {
          brush_size = 1;
        }
        break;
      }
    }
    if (draw) {
      drawSurface(surface, x, y, brush_size, color);
      // update setiap render pixelnya pada posisi y dan x
      // SDL_Rect rect = {x, y, 20, 20};
      // SDL_FillRect(surface, &rect, 0x00fff000);
      SDL_UpdateWindowSurface(window);
    }

    // draw lagi agar berada di atas brus nya
    drawPalette(surface, color_palette, color_pallete_size);
    SDL_Delay(delai_milis);
  }
}
