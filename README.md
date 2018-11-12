https://github.com/axos88/out_dir_bug/blob/master/src/foo.rs#L1

Apparently that line causes env!("whatever") to resolve to "false", so the include! fails, because it cannot find the file.

I'm a bit baffled on why this doesn't create a more meaningful compile error. Seems like it gets confused due to there being a module named env and a macro named env! in std?

By completing that line to std::env, it will compile fine.

Actually the case is much more complex:

use env;

include!(concat!(env!("OUT_DIR"), "/data.rs"));


error: couldn't read "src/false/data.rs": No such file or directory (os error 2)
 --> src/foo.rs:3:1
  |
3 | include!(concat!(env!("OUT_DIR"), "/data.rs"));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
use env;

include!(concat!(std::env!("OUT_DIR"), "/data.rs"));

error: couldn't read "src/false/data.rs": No such file or directory (os error 2)
 --> src/foo.rs:3:1
  |
3 | include!(concat!(std::env!("OUT_DIR"), "/data.rs"));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
use std::env;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

error[E0432]: unresolved import `prelude`
 --> src/main.rs:6:5
  |
6 | use prelude::*;
  |     ^^^^^^^ Did you mean `std::prelude`?
Seems like this one is resolving the env! correctly

use std::env;

include!(concat!(std::env!("OUT_DIR"), "/data.rs"));

error: couldn't read "src/false/data.rs": No such file or directory (os error 2)
 --> src/foo.rs:3:1
  |
3 | include!(concat!(std::env!("OUT_DIR"), "/data.rs"));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
** NOW THE SAME WITH NON_EXISTING_ENV_VAR **

use env;

include!(concat!(env!("INVALID"), "/data.rs"));

error: couldn't read "src/false/data.rs": No such file or directory (os error 2)
 --> src/foo.rs:3:1
  |
3 | include!(concat!(std::env!("OUT_DIR"), "/data.rs"));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
use env;

include!(concat!(std::env!("INVALID"), "/data.rs"));

error: couldn't read "src/false/data.rs": No such file or directory (os error 2)
 --> src/foo.rs:3:1
  |
3 | include!(concat!(std::env!("INVALID"), "/data.rs"));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
use std::env;

include!(concat!(env!("INVALID"), "/data.rs"));

error: environment variable `INVALID` not defined
 --> src/foo.rs:3:18
  |
3 | include!(concat!(env!("INVALID"), "/data.rs"));
  |                  ^^^^^^^^^^^^^^^
error: couldn't read "src/0/data.rs": No such file or directory (os error 2)
 --> src/foo.rs:3:1
  |
3 | include!(concat!(env!("INVALID"), "/data.rs"));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
This one seems to try to resolve the invalid env var, and reports an error that it doesn't find it. BUT! It also tries to include src/0/data.rs?!!!

use std::env;

include!(concat!(std::env!("INVALID"), "/data.rs"));


error: couldn't read "src/false/data.rs": No such file or directory (os error 2)
 --> src/foo.rs:3:1
  |
3 | include!(concat!(std::env!("INVALID"), "/data.rs"));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
*** If the src/false/data.rs is created, they give a different result! ***

use std::env;

include!(concat!(std::env!("INVALID"), "/data.rs"));

error[E0432]: unresolved import `prelude`
 --> src/main.rs:6:5
  |
6 | use prelude::*;
  |     ^^^^^^^ Did you mean `std::prelude`?
error[E0433]: failed to resolve. Could not find `env` in `std`
 --> src/foo.rs:3:23
  |
3 | include!(concat!(std::env!("INVALID"), "/data.rs"));
  |                       ^^^ Could not find `env` in `std`

error: aborting due to 2 previous errors
Seems like it did determine the file exists, but then also said env doesn't exist in std?!
Without the file present it sais that it cannot find the file (and the only way to determine that if it has calculated the path), but if the path it is looking for exists, it reports that it cannot determine the path, because one the macros needed to determine it cannot be found?! This seems like a causality paradox!

*** If the src/0/data.rs is created, it doesn't complain about the prelude anymore,
just that the env file is missing, basically the same paradox as before ***

use std::env;

include!(concat!(env!("INVALID"), "/data.rs"));

error: environment variable `INVALID` not defined
 --> src/foo.rs:3:18
  |
3 | include!(concat!(env!("INVALID"), "/data.rs"));
  |                  ^^^^^^^^^^^^^^^

