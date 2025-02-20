# Work Diary

## 背景

需要每天写日志，发送给领导，所以说我想写一个自动化的工具，来帮我完成这个任务。

## 目标

每天维护一个文件夹，里面存放当天的日志，然后使用 crontab 自动发送邮件给领导。

## 实现

1. 创建一个文件夹，用于存放日志文件；
2. crontab 定时任务，每天 22:00 发送邮件；

如果日志文件没有内容，不发送邮件。
本工具支持本地以及 smb 协议，可以将日志文件保存到局域网中，但是使用了 pavao 的 smb 协议库，所以需要安装 smbclient。

macos 可以使用 brew 安装 smbclient：

```bash
brew install samba
```

windows 暂不清楚如何安装 smbclient。
