int main() {
  int foo = 0;
  for (int asd = 0; asd < 3; asd++) {
    foo += 1;
  }

  int hey = foo++;

  return hey + foo;
}