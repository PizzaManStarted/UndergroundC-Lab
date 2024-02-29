// Create a function and experiments with lambda
#include <iostream>

using namespace std;


void fct(){
  cout << "executeee";
}



int main (int argc, char *argv[]) {
  auto lambdaFun = [](){
    cout << "Hello i am a funny lambda" << endl;
  };

  lambdaFun();

  auto add = [](int a, int b) -> int{
    return a + b;
  };

  cout <<  "Sum = " << add(1, 21) << endl;

  int num = 0;
  auto increment_by_one = [&num] () {
    num ++;
  };
  
  increment_by_one();
  cout << "increment testing : " << num << endl;


  
  increment_by_one();
  cout << "increment testing : " << num << endl;


  increment_by_one();
  cout << "increment testing : " << num << endl;


  increment_by_one();
  cout << "increment testing : " << num << endl;


  return 0;
}
