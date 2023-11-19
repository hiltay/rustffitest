#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>


extern int32_t
test();

int main(void) {


  int32_t pop1 = test();

  printf("%" PRId32 "\n", pop1);
}