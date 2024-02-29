#include <iostream>


using namespace std;

int main (int argc, char *argv[]) {
  // Image
  int image_width = 500;
  int image_height = 500;



  // Render
  cout << "P3\n" << image_width << " " << image_height << "\n255\n";

  for (unsigned int j = 0; j < image_height; j++) {
    clog << "\rScanlines remaining: " << (image_height - j) << " " << flush;
    for (unsigned int i = 0; i < image_width; i++) {
      auto r = double(i) / (image_width-1);
      auto g = double(j) / (image_height-1);
      auto b = double(i*j) / (image_height * image_width );

      int ir = static_cast<int>(255.999 * r);
      int ig = static_cast<int>(255.999 * g);
      int ib = static_cast<int>(255.999 * b);

      cout << ir << ' ' << ig << ' ' << ib << '\n';
    }
  }
  clog << "\rDone.              \n";
  return 0;
}
