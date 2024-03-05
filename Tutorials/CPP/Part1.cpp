#include <iostream>
// Math library
#include <cmath>


using namespace std;

int main (int argc, char *argv[]) {
  int x = 10;
  x ++;
  int y = x;
  // endl === "\n"
  cout << "x = " << x << endl;
  
  cout << "Give me an int: ";
  int input_value;
  // User inputs
  cin >> input_value;
  
  cout << "I got: " << input_value << endl; 


  cout << "Now give me an X and Y: ";
  x= 0;
  y = 0;
  cin >> x >> y;

  cout << "X: " << x << " | Y: " << y << endl;
  
  // Funny maths
  cout << "Sin(x): ";
  int sinR = 0;
  
  cin >> sinR;

  cout << "Result is: " << sin(sinR) << endl;

  return 0;
}
