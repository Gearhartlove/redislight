# redislight
A small redis repl 
## Highlights
This project features five commands, key/value expiration, an organized CLI structure,
a detailed parser, and error messages. 

The commands are : SET |  DEL |  GET  |  LPUSH  | LPOP |  LRANGE

Each command has different required/optional arguments and sometimes subcommands. 
Any command's arguments and subcommands can be queried  with the `[COMMAND] -h`. 

```
>> SET -h
Set 'key' to hold the string 'value'

Usage: SET <KEY> <VALUE> [COMMAND]

Commands:
  EX       Set the specified expire time, in seconds
  PX       Set the specified expire time, in milliseconds
  EXAT     Set the specified Unix time at which the key will expire, in seconds
  PXAT     Set the specified Unix time at which the key will expire, in milliseconds
  NX       Only set the key if it does not already exist
  XX       Only set the key if it already exist
  KEEPTTL  Retain the time to live associated with the key
  GET      Return the old string stored at key, or nil if key did not exist. An error is returned and SET aborted if the value stored at key is not a string
  help     Print this message or the help of the given subcommand(s)

Arguments:
  <KEY>    Determines where in the table the data will be stored
  <VALUE>  Determines what will be stored at a given table location

Options:
  -h, --help  Print help information
```
Detailed Documentation can be generated with `cargo doc --open`

A user can define a key/value pair to expire from the database in X seconds/milliseconds.
```
>> SET name Kristoff EX 60   // set the key/value (name/"kristoff) to expire in 60 seconds
```
Helpful error messages present themselves when the user provides incorrect input.
```
>> notacommand
error: The subcommand 'notacommand' wasn't recognized

Usage: redislight <COMMAND>

For more information try '--help'

(invalid command)
>>
```
## Assumptions
I assumed a number of things while working on this project. I will list them off below in a bulleted fashion.
* I only have one hash map which stores every key value pair, regardless of type 
* My keys would always be a string
* The underlying data structure of the value would also be String (also including Linked Lists with strings)
* User would start program by running 'cargo run' and not the name of the application
* I made a design decision to not allow negative indexing in LRANGE because I think they are wildly unclear and they go against my best coding practices
* I only need to store Strings and Linked Lists in the hash map's values to support the available five commands
## Commands
#### SET  GET  DEL
```
>> GET nonexisting
(nil)
>> SET mykey "Hello World!"
OK
>> GET mykey
Hello World!
>> SET name Kristoff
OK
>> DEL mykey name notakey
(integer) 2
>> GET mykey
(nil)
>> GET name
(nil)
>>
```
#### SET Subcommands
```
>> SET state Montana 
OK
>> SET state Alaska GET           // returns the old value while assigning the new value
Montana
OK
>> GET state
Alaska
>> SET state Washington EX 10    // set the time to live to 10 seconds
OK
>> GET state
Washington
>> GET state                     // wait 10+ seconds
(nil)
```
#### LPUSH  LPOP  LRANGE
```
>> LPUSH list four three two one
(integer) 4
>> LPOP list
1) one
>> LPOP list 2
1) two
2) three
>> LPUSH list five six          // push more than one value at a time
(integer) 3
>> LRANGE list 0 5              // no out of bounds index error
1) six
2) five
3) four
>>
```
