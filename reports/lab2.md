# lab1

> 贺鲲鹏 无 85 2018011169

## 实现简述

- `AppManagerInner` 添加 `check_read_memory` 函数，当传入的地址不在用户程序可访问的空间时返回 `Err(-1)`
- `syscall::fs::sys_write` 在执行写操作前先调用 `check_read_memory` 进行地址检查
- `syscall::fs::sys_write` 在遇到错误后不再 `panic` ，而是使用 `error!` 将错误进行输出并返回 `-1`
- 修改所有内核不重要的输出为 `debug!`
- 在 `build.rs` 中添加测例的路径

## 思考题

### 1

执行三个测例时的输出如下

```log
[rustsbi] RustSBI version 0.2.0-alpha.1
...
[DEBUG] : [kernel] num_app = 3
[DEBUG] : [kernel] app_0 [0x8020a028, 0x8020c468)
[DEBUG] : [kernel] app_1 [0x8020c468, 0x8020e960)
[DEBUG] : [kernel] app_2 [0x8020e960, 0x80210d98)
[DEBUG] : [kernel] Loading app_0
scause=0x2
[DEBUG] : [kernel] IllegalInstruction in application, core dumped.
[DEBUG] : [kernel] Loading app_1
scause=0x2
[DEBUG] : [kernel] IllegalInstruction in application, core dumped.
[DEBUG] : [kernel] Loading app_2
[DEBUG] : [kernel] PageFault in application, core dumped.
[kernel] Panicked at src/batch.rs:72 All applications completed!
```

在用户态程序尝试使用 S 态操作时，cpu 会将相应的异常信息保存在 `scause` 等寄存器中，并跳转至 OS 的相应处理部分。在本实验中，跳转到了 `trap.S` 中的 `__alltraps` 标志处，经过寄存器的保存后进入 `trap_handler` 根据异常类型进行处理。三个测例中，前两个是非法指令，第三个是非法地址，分别输出了 `trap_handler` 中设定好的输出。

### 2

#### 1

`a0` 是内核栈的栈顶地址。

`__restore` 的两种使用场景是：

1. 在 `trap` 执行完成后恢复寄存器并跳回用户程序
2. 在用户程序执行前进行一系列初始化工作，包括程序入口点的设置、切换至用户态等

#### 2

这六行使用 `__alltraps` 结尾保存的数据重新设置了 `sstatus`、`sepc`、`sscratch` 三个寄存器。

`sstatus` 存储的是 CPU 所在特权级的信息，修改后之后执行 `sret` 返回用户态。

`sepc` 存储 `trap` 发生前的指令地址，修改后用于后续跳回用户程序继续执行。换言之其是 `trap` 的 `ra`。

`sscratch` 修改后为用户栈的指针，之后使用 `csrrw` 与 `sp` 交换回到用户栈。

#### 3

`x2` 和 `x4` 分别对应 `sp` 和 `tp`，其中栈指针 `sp` 后面会用到，而 `tp` 由于目前没有程序需要使用所以并不需要保存和恢复。

#### 4

该指令执行后，`sp` 中为用户栈指针，`sscratch` 中为内核栈指针，即程序状态全部恢复，下一步跳转回用户程序。

#### 5

发生在 `sret`。该指令执行后，CPU 会设置 CSR 寄存器的值返回用户态。

#### 6

`csrrw sp, sscratch, sp` 交互两寄存器，使 `sp` 指向内核栈，用于 S 态的相关处理。

#### 7

在用户程序触发异常时，就已经进入 S 态了。对于一般的没有错误的用户程序，是 `ecall`。

### 3

riscv 支持 16 种异常和 6 种中断。中断包括 S/M 的软件中断、时钟中断和外部中断，异常包括非法指令、断点、错误地址、存读权限、存读页、ecall 等。

中断和异常通过 `mcause` 寄存器的 `XLEN-1` 位为 1 还是 0 进行判断。中断和异常的具体信息通过 `mcause` 的 `XLEN-2:0` 的内容进行判断。

在陷入 S 态时，比较重要的寄存器为：

- sstatus：SPP 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息
- sepc：当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址
- scause：描述 Trap 的原因
- stval：给出 Trap 附加信息
- stvec：控制 Trap 处理代码的入口地址

实际硬件中，这些寄存器和 m 态的 CSR 可能在一处。

### 4

部分中断可能可以确定不需要使用某些寄存器，从而可以不用保存（例如本实验种的 `x4(tp)` ）。可以考虑在进入 `__alltraps` 的位置加入一个中断类型向量，根据具体的中断跳转不同的寄存器保存与恢复代码。通过牺牲指令存储空间换取时间。
