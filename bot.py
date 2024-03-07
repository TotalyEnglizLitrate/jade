#!/usr/bin/python
from sys import argv, exit
from pathlib import Path
from subprocess import Popen
from datetime import datetime

lock_file = Path("/home/engliz/Documents/repos/jade/bot.lock")
log_file = Path(f"/home/engliz/Documents/repos/jade/logs/log_{datetime.now()}")
log_file_handle = log_file.open("w")
if len(argv) == 1: exit("Nothing to do")

match argv[1].lower():
    case "start":
        if lock_file.exists():
            exit("Bot already online")
        proc = Popen("/home/engliz/.cargo/bin/cargo-shuttle run --release", cwd=lock_file.parent, stdout=log_file_handle, stderr=log_file_handle, shell=True)
        lock_file.touch()
        lock_file.write_text(str(proc.pid))
        print("Bot is online")
    case "stop":
        if not lock_file.exists():
            exit("Bot not online")
        Popen(f"/usr/bin/kill -INT {lock_file.read_text().rstrip()}", shell=True)
        lock_file.unlink()
        print("Bot is offline")
    case "restart":
        if lock_file.exists():
            Popen(f"/usr/bin/kill -INT {lock_file.read_text().rstrip()}", shell=True)
            lock_file.unlink()
        print("Bot is offline")
        proc = Popen("/home/engliz/.cargo/bin/cargo-shuttle run --release", cwd=lock_file.parent, stdout=log_file_handle, stderr=log_file_handle, shell=True)
        lock_file.touch()
        lock_file.write_text(str(proc.pid))
        print("Bot is online")
    case _:
        log_file_handle.close()
        log_file.unlink()
        exit("Nothing to do")
