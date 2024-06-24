# charsplit

charsplit is a small utility tool that will give you information
about your string input. It will split the string into its bytes
and graphemes, and give you information about them.

## Usage

```bash
$ echo "Hello and привет" | charsplit
```
will output:
```
 Grapheme  Byte in binary  Byte in decimal  Unicode in decimal  Unicode in hex  Byte Type         Unicode Group
 H         1001000         72               72                  0x48            Ascii             Basic Latin (LatinAlphabet:Uppercase)
 e         1100101         101              101                 0x65            Ascii             Basic Latin (LatinAlphabet:Lowercase)
 l         1101100         108              108                 0x6C            Ascii             Basic Latin (LatinAlphabet:Lowercase)
 l         1101100         108              108                 0x6C            Ascii             Basic Latin (LatinAlphabet:Lowercase)
 o         1101111         111              111                 0x6F            Ascii             Basic Latin (LatinAlphabet:Lowercase)
           100000          32               32                  0x20            Ascii             Basic Latin (ASCIIPunctuation& Symbols)
 a         1100001         97               97                  0x61            Ascii             Basic Latin (LatinAlphabet:Lowercase)
 n         1101110         110              110                 0x6E            Ascii             Basic Latin (LatinAlphabet:Lowercase)
 d         1100100         100              100                 0x64            Ascii             Basic Latin (LatinAlphabet:Lowercase)
           100000          32               32                  0x20            Ascii             Basic Latin (ASCIIPunctuation& Symbols)
 п         11010000        208              1087                0x43F           Utf8Base          Cyrillic
           10111111        191                                                  Utf8Continuation
 р         11010001        209              1088                0x440           Utf8Base          Cyrillic
           10000000        128                                                  Utf8Continuation
 и         11010000        208              1080                0x438           Utf8Base          Cyrillic
           10111000        184                                                  Utf8Continuation
 в         11010000        208              1074                0x432           Utf8Base          Cyrillic
           10110010        178                                                  Utf8Continuation
 е         11010000        208              1077                0x435           Utf8Base          Cyrillic
           10110101        181                                                  Utf8Continuation
 т         11010001        209              1090                0x442           Utf8Base          Cyrillic
           10000010        130                                                  Utf8Continuation
           1010            10               10                  0xA             AsciiNewLine      Control codes (C0)
```

