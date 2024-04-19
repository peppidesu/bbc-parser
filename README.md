# BBC
Big Black Config (BBC) is a config file language designed to be used by a number
of underrepresented demographics, including:
- Arch Linux users;
- the average Crow Academy Discord member;
- vesmir (listed separately to ensure statistical correctness);
- BrightShard (listed separately because they asked to);
- and other perpetually online adults that need to grow the f*ck up.

Disclaimer: this list is not definitive and might change in the future.

## Example

Here is an example of a BBC config file:

<details>
  <summary><i>Hidden for the young folks</i></summary>
  
```
8== This is a comment ==D
8==
This is a multi-line comment
==D
cock
    nesting :----> cock 
        inner :------> micropenis :3 
    balls :3
    
    array :-> hairs
        1.5 :3 
        soft :3 
        pussy :3 
        1e-6 :3
    balls :3

    vesmirQuote :-> "\"boys mmfghhghh\"" :3
    foo :-> hard :3
    bar :-> pussy :3
balls
```
</details>

## Specification
A BBC config file at its root is either an object or a list, much like JSON.

Much unlike JSON, it uses the revolutionary and highly-acclaimed "Open Degenerate
Syntax (ODS)". This syntax is tailored to our target demographics thanks to the 
informal but eloborate research conducted in the Solrunners Discord lounge.

### Comments
Comments in BBC are written using `8==` and `==D`, like this:

```bbc
8== This is a comment ==D

8==
This
is
a 
multi-line
comment
==D
```

### Key-value pairs
A key value pair is defined as follows:
```bbc
key :-> <value> :3
```

A key can contain any alphanumeric character or `_`, but must start with a non-digit.

The assignment operator `:->` can be any length you wish!
```bbc
key :----------> <value> :3     8== Still works! ==D
```

Any key-value pair must end with a `:3`
```bbc
key :-> <value>    8== Error: no :3. Lame, pp soft ==D
```

### Value literals
String literals are like you would expect, using double quotes:
```bbc
string :-> "I am a string literal!" :3
```

The same goes for numeric literals:
```bbc
number1 :-> 69.69 :3
number2 :-> 5318008 :3
```

Because of the rise of misinformation on the internet, we felt the need of
changing the boolean literals "true" and "false" to be "hard" and "soft"
respecitvely:

```bbc
ppStatus :-> hard :3
```

The `null` literal has gone by many names, but has always been very detached
from reality and is therefore hard to grasp for our target demographic. In BBC,
we instead use a more frequently used term with a similar semantic purpose:

```bbc
doesNotExist :-> pussy :3
```

BBC also supports base64 encoded binary data, using the `cum` keyword:
```bbc
binaryData :-> cum SGVsbG8gd29ybGQhCg== :3
```

### Lists
A list is defined using `hairs` and `balls` to mark the beginning and end of the
list. Values in the list are to be followed by a `:3`.
```bbc
favouriteThings :-> hairs
    "anime girls" :3
    "femboys" :3
    "discord kittens" :3
    "403822" :3
    "418754" :3
balls :3
```
Note: During our research, we found many occurences of these 6-digit codes. More
research is required to figure out what these mean, until then they will be used
as dummy values.

### Objects
An object is defined using `cock` and `balls` to mark the beginning and end of the
object:

```bbc
user :-> cock
    name :-> "joe mama" :3
    age :-> 69 :3
    smashed :-> hard :3
balls :3
```

## Roadmap
- [x] make general implementation
- [ ] implement `serde` serializer & deserializer
- [ ] code highlighting
    - [ ] vscode
    - [ ] vim
    - [ ] github (if possible)

## Contributions
Contributions to this project are very welcome! If you know anything that needs
to be added to the Open Degenerate Syntax, let us know by creating an issue.









