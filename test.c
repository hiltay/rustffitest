// #include <stdio.h>
#include <stdint.h>
// #include <inttypes.h>


extern int test();

int main(void) {


  int pop1 = test();

  printf("%d \n", pop1);
}