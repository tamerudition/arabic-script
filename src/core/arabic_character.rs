/**************************************************************************************************
 * Copyright 2023 Tamer Elzein <tamer@tamerudition.com>                                           *
 *                                                                                                *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file      *
 * except in compliance with the License. You may obtain a copy of the License at                 *
 *                                                                                                *
 * http://www.apache.org/licenses/LICENSE-2.0                                                     *
 *                                                                                                *
 * Unless required by applicable law or agreed to in writing, software distributed under the      *
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,      *
 * either express or implied. See the License for the specific language governing permissions     *
 * and limitations under the License.                                                             *
 **************************************************************************************************/

use std::fmt::{Debug, Display, Formatter};

use crate::core::UnicodeCharacter;
use crate::ArabicLetterDad;

/// Represents a character of the Arabic script.
///
/// An Arabic character can be instantiated directly by specifying one of its variants by name:
///
/// ```
/// let character = arabic_script::ArabicLetterDad;
/// ```
///
/// Alternatively, it can be instantiated from a [`char`]...
///
/// ```
/// let character = arabic_script::ArabicCharacter::try_from('ض').unwrap();
/// ```
///
/// Note that the [`ArabicCharacter::try_from()`] method will return an [Error][`Err`] for any
/// character that does not belong to the Arabic script.
pub enum ArabicCharacter {
    ArabicLetterDad,
}

impl ArabicCharacter {
    /// Returns the underlying Unicode character.
    fn character(&self) -> impl UnicodeCharacter {
        match self {
            ArabicLetterDad => crate::core::characters::ArabicLetterDad::new(),
        }
    }
}

impl UnicodeCharacter for ArabicCharacter {
    fn block(&self) -> &'static str {
        self.character().block()
    }

    fn name(&self) -> &'static str {
        self.character().name()
    }

    fn scalar_value(&self) -> char {
        self.character().scalar_value()
    }
}

/**************************************************************************************************
 * Standard Library Implementations.                                                              *
 **************************************************************************************************/

impl Debug for ArabicCharacter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{ {} }}", self.name(), self.scalar_value())
    }
}

impl Display for ArabicCharacter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.scalar_value())
    }
}

impl PartialEq for ArabicCharacter {
    fn eq(&self, other: &Self) -> bool {
        self.block() == other.block()
            && self.name() == other.name()
            && self.scalar_value() == other.scalar_value()
    }
}

impl PartialEq<char> for ArabicCharacter {
    fn eq(&self, other: &char) -> bool {
        &self.scalar_value() == other
    }
}

impl PartialEq<&str> for ArabicCharacter {
    fn eq(&self, other: &&str) -> bool {
        &self.scalar_value().to_string() == other
    }
}

impl PartialEq<String> for ArabicCharacter {
    fn eq(&self, other: &String) -> bool {
        &self.scalar_value().to_string() == other
    }
}

impl TryFrom<char> for ArabicCharacter {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '\u{0636}' => Ok(ArabicLetterDad),
            _ => Err(()),
        }
    }
}

/**************************************************************************************************
 * Tests.                                                                                         *
 **************************************************************************************************/

#[cfg(test)]
mod tests {
    use crate::core::UnicodeCharacter;
    use crate::ArabicCharacter;

    use super::ArabicLetterDad;

    #[test]
    #[should_panic]
    fn accepts_only_arabic_script_characters() {
        ArabicCharacter::try_from('a').unwrap();
    }

    #[test]
    fn arabic_letter_dad() {
        assert_eq!(ArabicLetterDad.block(), "Arabic");
        assert_eq!(ArabicLetterDad.name(), "Arabic Letter Dad");
        assert_eq!(ArabicLetterDad.scalar_value(), '\u{0636}');

        assert_eq!(
            format!("{:?}", ArabicLetterDad),
            "Arabic Letter Dad { \u{0636} }"
        );
        assert_eq!(format!("{}", ArabicLetterDad), "\u{0636}");
        assert_eq!(ArabicLetterDad, ArabicLetterDad);
        assert_eq!(ArabicLetterDad, '\u{0636}');
        assert_eq!(ArabicLetterDad, "\u{0636}");
        assert_eq!(ArabicLetterDad, "\u{0636}".to_string());
        assert_eq!(
            ArabicLetterDad,
            ArabicCharacter::try_from('\u{0636}').unwrap()
        );
    }
}
