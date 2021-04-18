# lab 7

> 贺鲲鹏 2018011169

## 实现简述

- `DiskInode` 添加 `nlink` 项管理硬链接数量，`Inode` 添加获取信息的相关函数
- 实现 `sys_link_at` 和 `sys_unlink_at`
- `File` trait 添加 `stat` 函数用于获取文件信息，目前对于 `Stdin` `Pipe` 等返回默认空信息
- 修改 `syscall` 为五个参数

## 思考题

### 1

文件目录也是文件，同样存在 `DirEntry` 即可。不过在访问时需要实现根据路径一层层寻找，而不能只是简单地遍历根路径的所有目录项进行名字匹配。

### 2

Ubuntu 系统中不允许建立目录的硬链接，但可以建立软链接的环路。建立后如下：

```bash
❯ ln /mnt/c/Users/duskmoon/Documents/课程/OS/link-test/a c
ln: /mnt/c/Users/duskmoon/Documents/课程/OS/link-test/a: hard link not allowed for directory

# /mnt/c/Users/duskmoon/Documents/课程/OS/link-test/a/b
❯ ls -al
total 0
drwxr-xr-x 1 ubuntu ubuntu 4096 Apr 18 21:44 .
drwxr-xr-x 1 ubuntu ubuntu 4096 Apr 18 21:34 ..
lrwxrwxrwx 1 ubuntu ubuntu   53 Apr 18 21:35 c -> /mnt/c/Users/duskmoon/Documents/课程/OS/link-test/a
lrwxrwxrwx 1 ubuntu ubuntu   53 Apr 18 21:44 d -> /mnt/c/Users/duskmoon/Documents/课程/OS/link-test/a
```

可以不停的进入下级目录，实现 `/a/b/c/b/c/b/c/b/...`。

如果建立硬链接的环路，需要不停遍历每一级目录的软件都会陷入环路中，导致卡死或结果错误。Ubuntu 的做法是不允许建立硬链接，而是使用具有明确标识的软链接。
