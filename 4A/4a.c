#include <stdio.h>
int main(void) {
    unsigned int weight;
    scanf("%u", &weight);
    if (weight > 3 && weight % 2 == 0) {
        printf("YES\n");
    } else {
        printf("NO\n");
    }
    return 0;
}