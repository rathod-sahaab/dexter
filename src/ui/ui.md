# UI design document for dexter
## Principles
1. Input is not time based, let's say your combination is `ABC` the delay/duration between `A->B->C` should not matter.

## Flow
1. Lock is in `locked` state
2. Enter your passcode, digit by digit.
   1. Progress counter increases
3. Once all digits are entered
   1. Success: Lock opens and is kept open, success light is on.
   2. Failure: Faliure light is on until password is tried again. Goto `1`
4. Once lock opens
   1. Press first key to lock again
   2. Press all keys to reset password (success and active light are on)
      1. Press all keys to cancel password change
      2. Once password changes lock again


## Designs
### Typestate pattern
States are `LOCKED`, `OPEN`, `PASSWORD_CHANGE`

Operations allowed in locked:
1. Enter password digit by digit.
2. Indicate success or failure.


### MVC
1. Model
2. View
   1. Success failure indicator
   2. Progress bar
   3. Activity indicator (blinking?)
3. Controller