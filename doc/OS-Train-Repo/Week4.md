# Week 4

## 制定测例测试

制作了兼容了 Unikernel 和宏内核架构的 OS 之后，一个比较重要的问题便是这种模块化的 OS 是否会对性能造成影响。因此需要对内核进行测试。



根据宏内核 和 Unikernel 的异同之处，考虑从以下几个方面来进行测例测试：

* 性能：
  * 文件：包括读写等内容
  * 任务：新建线程、任务切换、睡眠
  * 内存分配：动态分配堆内存、对内存空间的读写操作
* 安全性：
  * 非法访问地址测试
  * 堆栈空间溢出测试
  * 。。



测试对象：

1. ArceOS 本身
2. Starry 本身
4. 本地宏内核



## 测试结果

**若无特殊说明，下列提到的时间 单位均为 ns。**

### 文件 IO



反复对一个文件进行 打开、读或者写操作 50 次，计算消耗的时间，并重复该操作 10 次 计算平均值。



| 测例      | ArceOS     | Starry     |
| --------- | ---------- | ---------- |
| Fileopen  | 550282390  | 1066207190 |
| Fileread  | 615516760  | 1070663240 |
| Filewrite | 1690372150 | 2218262040 |



### 内存分配

测试操作分为两种：

A：连续申请内存，并一次性全部释放。

B：申请一次内存，并立即释放，然后继续申请下一次内存，不断重复。



进行 Num 次操作 A 和 操作 B，计算消耗的时间。

| Num    | 10     | 50      | 100     | 500      | 1000     | 5000      | 10000     |
| ------ | ------ | ------- | ------- | -------- | -------- | --------- | --------- |
| ArceOS | 474300 | 1843400 | 3748100 | 19115600 | 39338600 | 184836300 | 377351000 |
| Starry | 812000 | 2808000 | 5693700 | 28515300 | 57336100 | 335300400 | 556783200 |



### 任务调度

新建 50 个线程，每一个线程执行 Num 次 yield 函数，计算最后所有线程释放时经过的时间。并将该时间和本地WSL 运行结果对比：

| Num  | Starry   | ArceOS  | Linux   |
| ---- | -------- | ------- | ------- |
| 1    | 54763910 | 3436430 | 1151728 |
| 2    | 54694900 | 388390  | 869095  |
| 5    | 54971370 | 493690  | 881606  |
| 10   | 56682550 | 707700  | 939276  |
| 15   | 58620320 | 922290  | 885656  |
| 20   | 59653070 | 1195070 | 919943  |
| 25   | 60819050 | 1335540 | 902737  |
| 50   | 67460820 | 2258190 | 946170  |
| 60   | 70335530 | 2863900 | 929436  |
| 70   | 72744350 | 3285170 | 908032  |
| 80   | 75324650 | 3654020 | 1070883 |
| 90   | 77637570 | 4034390 | 1040678 |
| 100  | 80407590 | 4202130 | 1042760 |

发现 Linux 环境下随着 Num 变化，总时间几乎不变，且快于 ArceOS 环境，这是不太合理的。所以思考出现这种现象的原因。



尝试横向对比与纵向对比。

横向对比：对比其他的系统调用，看看是否是因为 yield 这个系统调用的问题导致 ArceOS 变慢。

测试 **getpid **系统调用。

| Num    | ArceOS  | Starry    | Linux   |
| ------ | ------- | --------- | ------- |
| 1000   | 61370   | 3108410   | 65998   |
| 2000   | 107820  | 6384370   | 113409  |
| 5000   | 266620  | 15653880  | 273750  |
| 10000  | 538550  | 31311610  | 565014  |
| 15000  | 812640  | 47527430  | 718736  |
| 20000  | 1224000 | 62507120  | 990795  |
| 25000  | 1303640 | 77740360  | 1209650 |
| 50000  | 2623760 | 155429130 | 2438787 |
| 600000 | 3360740 | 186802430 | 2950229 |
| 70000  | 3682740 | 218574090 | 3463541 |
| 80000  | 4175930 | 248619690 | 3979007 |
| 90000  | 4661770 | 279418020 | 4521014 |
| 100000 | 5395420 | 310067620 | 5000976 |

发现 ArceOS 和 Linux 时间接近，且 Linux 的运行时间与 Num 成线性关系。



纵向对比：思考是否会是调度策略的问题，因此在**单线程**环境下运行 Num 次 yield 调用，减少多线程带来的影响。

| Num    | Starry   | ArceOS  | Linux   |
| ------ | -------- | ------- | ------- |
| 100    | 580640   | 77440   | 13057   |
| 200    | 1151490  | 34590   | 23063   |
| 500    | 2864910  | 94620   | 53362   |
| 1000   | 5806980  | 170770  | 103447  |
| 1500   | 8752370  | 257950  | 158525  |
| 2000   | 11437370 | 350320  | 210769  |
| 2500   | 14163370 | 468890  | 259136  |
| 5000   | 28526900 | 940180  | 514902  |
| 6000   | 34547640 | 1061070 | 661440  |
| 7000   | 39532740 | 1183400 | 716446  |
| 8000   | 45511390 | 1355080 | 840507  |
| 9000   | 51267930 | 1665240 | 921227  |
| 100000 | 57452610 | 1771540 | 1026559 |

发现 Linux 的运行时间与 Num 成线性关系，且 ArceOS 和 Linux 的运行差距接近。



思考：

这种情况的出现可能有以下几个原因：

1. 本地 WSL 上的 Linux 内核运行的任务调度策略不一定是 ArceOS 和 Starry 默认采用的 FIFO 策略，可能是其他策略，同时新建 50 个线程进行 yield 时，不会真正出现 50 个线程同时存在的情况，从而加快了运行速度，使得运行时间不呈线性。
2. Linux 内核对 syscall 存在优化，如 sys_yield 也可以特判 syscall_id 然后从 trap_entry 那边直接跳到线程切换的汇编，而不需要拿一个inner锁、等待各种资源量等，加速了处理速度。但 ArceOS 并没有做这些优化。



### 异常处理

ArceOS 作为 Unikernel 架构，运行在单进程体系下，当出现如 SIGSEGV 错误时，会直接报 panic 并且退出内核。但是 Starry 可以在多进程体系下，捕获子进程的 SIGSEGV 信号并进行处理，而不需要退出整个程序。

相关测例在 ostrain/process_sigsegv，体现了宏内核运行的稳定性与安全性。