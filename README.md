# The Arabic Script Library

> An expressive API for the characters of the Arabic script.

**The Arabic Script Library** provides a clear, legible, and universally usable representation of
all the standard characters of the Arabic script.

Each letter, diacritical mark, numerical digit, and additional symbol is available under its
standardized Unicode name, providing easy access to all Arabic characters, with none of the hassle
of manipulating primitive Rust characters or Unicode code points.


## Motivation

The Arabic script has special orthographic rules and ligature forms. Many letters have a variety of
letter-forms. Diacritics are often necessary to disambiguate the pronunciation of some letters,
depending on their position in a word. Many non-standard symbols have been added over the years to
the Arabic script to represents entire phrases and expressions.

The sheer number of these variations makes it impractical to type those glyphs directly on any
regular keyboard. Moreover, not many typefaces contain all the characters in the various Arabic
Unicode blocks. The very names of these glyphs can be quite lengthy and confusing. This becomes even
more problematic when attempting to communicate the name of these glyphs across teams in a
predictable and consistent manner.

**The Arabic Script Library** addresses these issues by introducing the `ArabicCharacter` type,
whose function is to encapsulate Arabic characters, assign to them clean and standardized names, and
provide simple methods for converting to and from Rust-native textual types.

The advantages of this API are...

1. Instances of the `ArabicCharacter` are _guaranteed_ to represent only Arabic characters.
2. It is trivial to utilize an `ArabicCharacter` in any place where other textual types are used.
3. The standardized names of characters makes it much easier to refer to, and legibly identify, them
   in source code and other situations when a proper typeface is not available.

All the things you can do with this abstraction are things you can do, albeit with non-trivial
effort and mental gymnastics, using the Rust-native `char`. What this library provides are
clarity, consistency, and ease-of-use.


## Licenses

This project is licensed under the [Apache License, Version 2.0](LICENSE).
