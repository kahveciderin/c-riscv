int main() {
  int *null_ptr = 0;
  int *ptr2 = &*null_ptr;
  return ptr2;
}