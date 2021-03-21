# lab 3

> 贺鲲鹏 无 85 2018011169

## 实现简述

- 添加 `task.stride` 以调度，实现了一个 `Stride(usize)` 的比较函数
- 更改 `sys_get_time` 为基于传递 `TimeVal` 的实现
- 重新实现 `check_read_memory` 以检查 `sys_write` 是否越界
- 添加 `sys_set_priority` 以设置优先级
- 添加 `task.duration` 以记录程序用时，并杀死超时程序

## 思考题

### 1

进程切换发生在 `suspend_current_and_run_next()` 和 `exit_current_and_run_next()` 中，即在一个进程被暂停或退出后切换到下一个进程。具体的为“时间片结束”“主动 yield”“运行结束”“超时杀死”四种情况。

下一个运行进程的选取在加入 `task.stride` 后，通过遍历找到 `stride` 最小的运行。由于本节尚未引入堆的分配，且需要运行的程序较少，故采用了这种 `O(n)` 的做法。值得一提的是，rust 本身有一些高级函数，可以不用手动的实现类似选择排序中的扫描部分，而是采用以下方法（省略了判断任务状态的部分）：

```rust
(current + 1..current + self.num_app + 1)
    .min_by(|x: &usize, y: &usize| {
        inner.tasks[*x]
            .stride
            .partial_cmp(&inner.tasks[*y].stride)
            .unwrap()
    })
```

这样实现后，再结合思考题 3 中所要实现的比较函数（即实现 `Stride(usize)` 的 `PartialOrd` ），即可正确取出正确的进程。

目前遍历进程的方法是根据不可改变量 `num_app` 得到一个 `Range<usize>` ，再通过索引得到进程。这样的实现是不支持加入新进程的。如果考虑可以动态加入新进程，可能需要实现堆分配后，用类似 `Vec<Box<TaskControlBlock>>` 的结构作为进程池，并使用 `iterator` 进行遍历。每次有新进程时，添加到队列最后，但是新进程的 `stride` 不好设置。

### 2

#### 1

在目前没有不同，因为没有新进程的产生。

当有新进程产生时，ch3 的调度策略没有加入新进程到进程池的功能，会导致无法运行。如果支持加入新进程到进程池数组尾部，则不会发生下面一问中所说的“后创建先执行”的情况。

#### 2

C 的调度如下

```
_p1 p2 p3 [RUN p1]
p1 _p2 p3 [FINISH p2]
p1 p4 _p3 p5 [RUN p3, SPAWN p4, p5]
p1 p4 p3 _p5 [RUN p5]
_p1 p4 p3 p5 [RUN p1]
p1 _p4 p3 p5 [RUN p4]

SPAWN : p1 p2 p3 p4 p5
 EXEC : p1 p2 p3 p5 p1 p4
```

在我实现的 stride 下，假定进程优先级相同为 $2^31$，生成进程时初始 `stride = 16`，`BIG_STRIDE = usize::MAX = 4294967295` 这里取 32 位机，这样每个进程 stride 更新步长为 1。执行情况列表得：

| time | p1  | p2  | p3  | p4  | p5  | status               |
| ---- | --- | --- | --- | --- | --- | -------------------- |
| 0    | 17  | 16  | 16  | NAN | NAN | RUN p1               |
| 1    | 17  | NAN | 16  | NAN | NAN | FINISH p2            |
| 2    | 17  | NAN | 17  | 16  | 16  | RUN p3, SPAWN p4, p5 |
| 3    | 17  | NAN | 17  | 17  | 16  | RUN p4               |
| 4    | 17  | NAN | 17  | 17  | 17  | RUN p5               |

```
SPAWN : p1 p2 p3 p4 p5
 EXEC : p1 p2 p3 p4 p5
```

### 3

如果不实现特殊的比较函数，则是 p2 先执行，因为 p2 执行后溢出，stride 变为 4。

考虑反证法，假设某一时刻第一次出现了 `p1.stride - p2.stride > BIG_STRIDE / 2`，则该时刻应执行 p1，否则上一时刻就是这种情况。那么执行前，`p1.stride_prev = p1.stride - p1.pass`，即有 `p1.stride_prev >= p1.stride - BIG_STRIDE / 2 > p2.stride = p2.stride_prev`，后面的等号由本时刻执行 p1 得到。因而有 `p1.stride_prev > p2.stride_prev`，则不可能执行 p1，矛盾，证毕。

实现中，`stride` 可以相等，且下述代码以 `usize` 进行存储：

```rust
#[derive(PartialEq)]
pub struct Stride(pub usize);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        ((self.0 - other.0) as isize).partial_cmp(&0)
    }
}

impl Stride {
    pub fn update(&mut self, priority: isize) {
        if priority <= 1 {
            panic!("Priority must larger then 1");
        } else {
            self.0 += BIG_STRIDE / priority as usize;
        }
    }
}
```

## 意见

希望教程和测例能统一，我这次还改了 user 里的 `syscall` orz
