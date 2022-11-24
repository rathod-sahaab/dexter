# Dexter

Dexter is an electronic lock. The way to open and close this lock is what makes it unique. Instead of pressing one key at a time you push multiple.

Normal pass-code/pin locks use a mechanism where you can activate one key/switch at a time, for this you need a lot more keys/switches for given range.

Example:

```
1   2   3
4   5   6
7   8   9
    0
```

Here you can have 10 keys giving you 0-9 and then you can chain those into some thing like `0-9-1-8-2`. This is waste of resources.

Dexter takes a better approach where you can press multiple keys a once to create a combination. Example with 5 keys your key pad will look like.

```
 1 1 1 1 1
```

_--- or ---_

```
16 8 4 2 1
```

Now you can enter binary number between `1-30` for example

| To enter | Combination |
| -------- | ----------- |
| 10       | `01010`     |
| 15       | `01111`     |
| 27       | `11011`     |

5 bit range is `0-31` Why not 0? You can't press no keys and register it as a key press. To enter 0 you need complex timing mechanism or extra key which you can use better to double the range.

You need significantly less number of keys but more **Dexterity** , hence the name **Dexter**.

So with same number of keys you can achieve more range of number, mathematically.

$$
range\ increased = \frac{\left(2^n - 2\right)}{n}\ times
$$

Which is near exponential increase.

## Usage

### Philosophy

Dexter user experience is designed keeping few things in mind

1. Humans make mistakes.
2. Humans mostly realise they have made a mistake halfway through.
3. Humans try to course correct the mistake.

### Structure

```
(3) (4)
(5) (6) (7) (8) (9)...

(1) (2) [ 1 ] [ 2 ] [ 3 ] [ 4 ] [ 5 ]
```

**LED indicators:**

- `(1)` Activity Indicator
- `(2)` Digit Registered Indicator
- `(3)` Success Indicator
- `(4)` Wrong Password Indicator
- `(5+)` Digits entered Indicator

**Keys:**

- `[1-5]` Keys ordered in MSB order

### Workflows

**Login Flow:**

1. Initially the lock is in standby, `(1)` is off, to activate it press first key then `(1)` pops up.
2. Start with your combination first digit in binary say `01101` when your are holding down given combination `(2)` is blinking indicating you can press more keys.
3. When you lift your hand up, `(2)` stops blinking momentarily indicating your digit input has been recorded. One more LED lights up in progress bar made up of `(5+)`
4. Once you enter correct password, lock opens and `(3)` stays lit up.
5. Incase of wrong password, `(4)` lights up for a few seconds.

**Password Change flow:**

1. Once lock is unlocked, press all keys, `(3)` starts blinking, now you can enter combination like in the login flow.
2. Fill all the digits indicated by `(5+)` progress bar.
3. **NOTE:** You can't use `00000` and `11111` in this passcode

**Forgot Password:**
1. Break the lock, if you were smart enough to created a reset button press that enter default password.
