
class MyClass{
  public:
    int myNum;

    MyClass(int myNum){
      this->myNum = myNum;
    }
};

int main(int argc, char const *argv[])
{
  /* code */
  MyClass obj(3);
  return 0;
}
