int main() {
  int hello = 20;
  int world = 4;
  while (world) {
    hello -= 1;
    world -= 2;
    break;
  }
  while (hello < 20) {
    hello += 1;
    continue;
    hello += 42;
  }
  return hello;
}