#include "signal.h"
#include "stdlib.h"
#include <cstdlib>
#include <iostream>

static volatile int flag = 1;

void handler(int dummy) { flag = 0; }

int main(void) {
  signal(SIGINT, handler);
  std::cout << "C";
  while (flag)
    std::cout << ">>";
  std::cout << " java" << std::endl;
  std::cout << "not so secret your repo :D" << std::endl;
  exit(EXIT_SUCCESS);
}
