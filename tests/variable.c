int main() {
  int world = 5;
  {
    int world = 2;
  }
  return world;
}