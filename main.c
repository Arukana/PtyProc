#include <stdio.h>
#include <sys/types.h>
#include <unistd.h>

int main() {
  pid_t pid = fork();
    if(pid == 0) {
      printf("this is a child: my new unique pid is %d\n", getpid());
    } else {
      printf("this is the parent: my pid is %d and I have a child with pid %d \n", getpid(), pid);
    }
}
