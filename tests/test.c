int main() {
  int hello = 23 ? 1 ? 2 : 3 : 4 ? 5 : 6;
  while (hello) {
    hello--;
  }
  return hello;
}