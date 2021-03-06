#include <stdio.h>
#include <stdint.h>

char* toBinary(int n, int len)
{
  char* binary = (char*)malloc(sizeof(char) * len);
  int k = 0;
  for (unsigned i = (1 << len - 1); i > 0; i = i / 2) {
    binary[k++] = (n & i) ? '1' : '0';
  }
  binary[k] = '\0';
  return binary;
}

static uint8_t bits (uint8_t *V, int i, int b) {
  int w0 = i/8, w1 = (i+b-1)/8, ii = i%8;
  uint8_t x = V[w0] >> ii;
  if (w0 < w1)
    x ^= V[w1] << (8 - ii);
  x &= (1UL<<b)-1;
  return x;
}

// This adds rows a little more efficiently since it
// doesn't add the words we have already zeroed out
static void addrows (uint8_t * restrict A, uint8_t * restrict B, uint8_t * restrict C, int s, int wds) {
  for (int w = s; w < wds; w++)
    A[w] = B[w] ^ C[w];
}

int main()
{
  int s   = 6;
  int S   = 3;
  int m   = 11;
  int wds = 2;
  const int SS = (1<<S);
  printf("SS is %d\n", SS);

  uint8_t arr[11][2] = {{128,   0}
			,{ 64,   0}
			,{ 32,   0}
			,{ 16,   0}
			,{  8,   0}
			,{  4,   0}
			,{  3, 255} // s here
			,{  1, 255}
			,{  0, 255}
			,{  3, 127}
			,{  2, 255}
  };

  // print binary matrix, spaces separate words
  for (int i = 0; i < m; i++) {
    printf("[ ");
    for(int j = 0; j < 2; j++) {
      char* binary = toBinary(arr[i][j], 8);
      printf("%s ", binary);
    }
    printf("]\n");
  };

  uint8_t (*Z)[wds] = malloc(SS * sizeof *Z); // Z[SS][wds]
  int *z = malloc(SS * sizeof *z);

  // first, clear Z[0]
  z[0] = 0;
  for (int w = s/8; w < 2; w++)
    Z[0][w] = 0;

  // now, for each pivot 0,...,S-1
  for (int i = 0; i < S; i++) {
    int ii = 1<<i;
    int vv = bits(&(arr[s+i]), s, S);
    // copy block of size 2^i and xor i-th row onto it
    for (int j = 0; j < ii; j++) {
      int a   = z[j];
      int b   = a ^ vv;
      z[j+ii] = b;
      addrows (Z[b], Z[a], &(arr[s+i]), s/8, wds);
    }
  }

  for (int i = s + S; i < m; i++) {
    int c = bits(&(arr[i]), s, S);
    addrows (&(arr[i]), &(arr[i]), Z[c], s/64, wds);
  }

  for (int i = 0; i < m; i++) {
    printf("[ ");
    for(int j = 0; j < 2; j++) {
      char* binary = toBinary(arr[i][j], 8);
      printf("%s ", binary);
    }
    printf("]\n");
  };
}
