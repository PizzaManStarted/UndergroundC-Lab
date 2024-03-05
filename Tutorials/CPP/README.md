# Learning C++


## Classes

Let's say we want to create the class "MyClass"

```c++
class MyClass{
  public:
    int myNum;
    string myString;
};
```

As for Java (and other less cool languages) we can use **access specifier** such as *public and private* to specify that the attributes and methods are accessible or not from outside the class.

Also notice that the semicolon at the end is mamdatory

We can initiate an object by doing:

```c++
MyClass obj;

obj.myNum = 3;
obj.myString = "hi";
```

But we could also add a constructor to our class

```c++
class MyClass{
  private:
    int myNum;
  
  public:
    MyClass(int myNum){
      this->myNum = myNum;
    }
};
int main (int argc, char *argv[]) {
  MyClass obj(3);
}
```
Here we have a private field for the *myNum* attribute and a constructor

* It is now the default constructor, meaning we **have** to pass an argument to instantiate an object.
