# 🟢 `[WORM-011]` Traceback in dashboard

- **UUID**: `e11627c9-9cf6-4ddf-9ca0-57959b0fedbe`
- **ID**: `WORM-011`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `dashboard`
- **Created**: `2026-10-04 16:43:17 UTC`
- **Resolved**: `2026-10-04 16:57:36 UTC`

#### Description

Traceback after leaving dashboard running overnight

#### Execution / Console Logs

```text
File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64052)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64053)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64055)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64056)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64057)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64058)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64059)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64060)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64061)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64062)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64063)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64064)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64065)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64066)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64067)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64068)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64069)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64070)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64071)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64072)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64073)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64074)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64075)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64076)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64077)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64078)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64079)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64080)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64081)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64082)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64083)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64084)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64085)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64086)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64087)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64088)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64089)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64090)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64092)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64093)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64094)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64095)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64096)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64097)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64098)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64099)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64100)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64101)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64102)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64103)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64105)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64107)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64108)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64109)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64110)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64111)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64112)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64113)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64114)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64115)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64116)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64117)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64118)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64119)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64120)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64121)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64122)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64123)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64124)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64125)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64126)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64127)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64128)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64129)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64130)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64131)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64132)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64133)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64134)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64135)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64136)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64137)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64138)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64139)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64140)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64141)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64142)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64143)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64144)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64145)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64146)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64147)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64148)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64149)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64150)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64151)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64152)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64153)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64154)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64155)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64156)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64157)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64158)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64159)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64160)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64161)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64162)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64163)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64164)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64165)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64166)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64167)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64168)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64169)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64170)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64171)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64172)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64173)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64174)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64175)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64176)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64177)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64178)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64179)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64180)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64181)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64182)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64183)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64184)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64185)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64186)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64187)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64188)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64189)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64190)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64191)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64192)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64193)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64194)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64195)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64196)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64197)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64198)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64199)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64200)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64201)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64202)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64203)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64204)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64205)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64206)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64207)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64208)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64209)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64210)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64211)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64212)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64213)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64214)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64215)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64216)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64217)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64219)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64220)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64221)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64222)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64223)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64224)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64225)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64226)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64227)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64228)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64229)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64230)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64231)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64232)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64233)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64234)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64235)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64236)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64237)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64238)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64239)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64240)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64241)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64242)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64243)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 167, in do_GET
    files = self.server.git_engine.get_changed_files(commit, include_feedback=include_feedback)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 1412, in get_changed_files
    status_out = run_git_command(cmd_status, cwd=self.repo_root)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64244)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 167, in do_GET
    files = self.server.git_engine.get_changed_files(commit, include_feedback=include_feedback)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 1412, in get_changed_files
    status_out = run_git_command(cmd_status, cwd=self.repo_root)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64245)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 167, in do_GET
    files = self.server.git_engine.get_changed_files(commit, include_feedback=include_feedback)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 1412, in get_changed_files
    status_out = run_git_command(cmd_status, cwd=self.repo_root)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64246)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 167, in do_GET
    files = self.server.git_engine.get_changed_files(commit, include_feedback=include_feedback)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 1412, in get_changed_files
    status_out = run_git_command(cmd_status, cwd=self.repo_root)
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64247)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64248)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64249)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64250)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64251)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64252)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64253)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 64254)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 121, in do_GET
    session = self.server.build_session()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 1023, in build_session
    branches = self.git_engine.get_branches()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 197, in get_branches
    current_branch = self.get_current_branch()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 185, in get_current_branch
    branch = run_git_command(["branch", "--show-current"], cwd=self.repo_root).strip()
             ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/vcs/git_engine.py", line 63, in run_git_command
    result = subprocess.run(
        cmd,
    ...<4 lines>...
        check=False,
    )
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 554, in run
    with Popen(*popenargs, **kwargs) as process:
         ~~~~~^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1005, in __init__
    errread, errwrite) = self._get_handles(stdin, stdout, stderr)
                         ~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/subprocess.py", line 1765, in _get_handles
    errread, errwrite = os.pipe()
                        ~~~~~~~^^
OSError: [Errno 24] Too many open files
----------------------------------------
----------------------------------------
Exception occurred during processing of request from ('127.0.0.1', 49926)
Traceback (most recent call last):
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 697, in process_request_thread
    self.finish_request(request, client_address)
    ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 362, in finish_request
    self.RequestHandlerClass(request, client_address, self)
    ~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/socketserver.py", line 766, in __init__
    self.handle()
    ~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 65, in handle
    super().handle()
    ~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 447, in handle
    self.handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 58, in handle_one_request
    super().handle_one_request()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/http/server.py", line 435, in handle_one_request
    method()
    ~~~~~~^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 423, in do_POST
    self._handle_update_verdict(data)
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/dashboard/server.py", line 722, in _handle_update_verdict
    out_path = self.server.review_server.save_and_sync()
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/code_review/server.py", line 575, in save_and_sync
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/code_review/sqlite_store.py", line 254, in save_session
  File "/Users/<username>/miniforge3/envs/cq/lib/python3.13/contextlib.py", line 141, in __enter__
  File "/Users/<username>/gh/firmware/tools/dashboard/provider/code_review/sqlite_store.py", line 43, in _get_connection
sqlite3.OperationalError: unable to open database file
----------------------------------------
^C
Terminating neural link via interrupt signal...
Xerxes workstation offline. Terminal released.
```

#### Resolution Notes

Resolved unclosed SQLite connections in build_session() and ensured immediate connection closure in DashboardRequestHandler to prevent open file descriptor exhaustion.
