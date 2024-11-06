int main() {
  int a = 3;
  int b = 5;
  int *ptr = &a;
  *ptr = 4;
  ptr = &b;
  *ptr += 1;
  return a + b;
}