# lab 6

> 贺鲲鹏 2018011169

## 实现简述

- 添加 `MailRingBuffer` `MailBox` `Socket` 结构用于存储和发送邮件
  - `MailBox` 实现了 `read`
  - `Socket` 实现了 `write`，类似 `Pipe`
- 添加 `sys_mailread` `sys_mailwrite` 调用实现读写邮件
- 实现 `PidHandle` 和 `usize` 的比较，并添加 `find_task` 方法根据 `pid` 找到 `TCB`

## 思考题

### 1

查看占用 40001 端口的情况

```bash
❯ netstat -anp | grep 40001
(Not all processes could be identified, non-owned process info
 will not be shown, you would have to be root to see it all.)
tcp6       0      0 :::40001                :::*                    LISTEN      19/node
tcp6       0      0 172.30.164.69:40001     172.30.160.1:60550      ESTABLISHED 81/node
tcp6       0      0 172.30.164.69:40001     172.30.160.1:60549      ESTABLISHED 19/node
```

### 2

多核情境下，可能发生读写冲突，即不同核的进程同时读写同一个报文，导致环形缓冲区的头尾指针值发生读后写等顺序错误。单核多线程情况下也可能会发生这种问题，因为读写并非原子操作，可能会被时钟中断打断（视相关处理实现而定，框架会屏蔽中断）。

增加互斥锁（信号量），读写互斥，即读写不能同时发生。根据具体需求，读可以改为允许多线程同时访问，并且不负责邮件的删除，写及删除等修改操作则同时只允许一个进程进行。

可以仿照 TCP/IP，加入端口，收发邮件时需先建立对应的 socket 连接。这样不同的进程来信可以绑定到不同端口的 FIFO 队列上，交由不同的线程处理，减少同步互斥问题。

## 建议

框架中有大量的直接使用 `unwrap()` 等解构方法的地方，在这节明显的与测例的错误检测冲突。希望改为 `Result`，也让铜须理解 Rust 的异常处理机制。

此外，本次实验可能需要仔细思考锁，建议提供一些链接。
