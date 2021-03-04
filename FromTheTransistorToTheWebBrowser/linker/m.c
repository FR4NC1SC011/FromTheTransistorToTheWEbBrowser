extern void a(char *);

int main(int ac, char **av) {
  static char string[] = "Hello world\n";

  a(string);
}
