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
use crate::ArabicDamma;
use crate::ArabicDammatan;
use crate::ArabicFatha;
use crate::ArabicFathatan;
use crate::ArabicKasra;
use crate::ArabicKasratan;
use crate::ArabicLetterAin;
use crate::ArabicLetterAlef;
use crate::ArabicLetterAlefMaksura;
use crate::ArabicLetterAlefWithHamzaAbove;
use crate::ArabicLetterAlefWithHamzaBelow;
use crate::ArabicLetterAlefWithMaddaAbove;
use crate::ArabicLetterBeh;
use crate::ArabicLetterDad;
use crate::ArabicLetterDal;
use crate::ArabicLetterFeh;
use crate::ArabicLetterGhain;
use crate::ArabicLetterHah;
use crate::ArabicLetterHamza;
use crate::ArabicLetterHeh;
use crate::ArabicLetterJeem;
use crate::ArabicLetterKaf;
use crate::ArabicLetterKhah;
use crate::ArabicLetterLam;
use crate::ArabicLetterMeem;
use crate::ArabicLetterNoon;
use crate::ArabicLetterQaf;
use crate::ArabicLetterReh;
use crate::ArabicLetterSad;
use crate::ArabicLetterSeen;
use crate::ArabicLetterSheen;
use crate::ArabicLetterTah;
use crate::ArabicLetterTeh;
use crate::ArabicLetterTehMarbuta;
use crate::ArabicLetterThal;
use crate::ArabicLetterTheh;
use crate::ArabicLetterWaw;
use crate::ArabicLetterWawWithHamzaAbove;
use crate::ArabicLetterYeh;
use crate::ArabicLetterYehWithHamzaAbove;
use crate::ArabicLetterZah;
use crate::ArabicLetterZain;
use crate::ArabicShadda;
use crate::ArabicSukun;
use crate::ArabicTatweel;

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
    ArabicLetterHamza,
    ArabicLetterAlefWithMaddaAbove,
    ArabicLetterAlefWithHamzaAbove,
    ArabicLetterAlefWithHamzaBelow,
    ArabicLetterWawWithHamzaAbove,
    ArabicLetterYehWithHamzaAbove,
    ArabicLetterAlef,
    ArabicLetterBeh,
    ArabicLetterTehMarbuta,
    ArabicLetterTeh,
    ArabicLetterTheh,
    ArabicLetterJeem,
    ArabicLetterHah,
    ArabicLetterKhah,
    ArabicLetterDal,
    ArabicLetterThal,
    ArabicLetterReh,
    ArabicLetterZain,
    ArabicLetterSeen,
    ArabicLetterSheen,
    ArabicLetterSad,
    ArabicLetterDad,
    ArabicLetterTah,
    ArabicLetterZah,
    ArabicLetterAin,
    ArabicLetterGhain,
    ArabicTatweel,
    ArabicLetterFeh,
    ArabicLetterQaf,
    ArabicLetterKaf,
    ArabicLetterLam,
    ArabicLetterMeem,
    ArabicLetterNoon,
    ArabicLetterHeh,
    ArabicLetterWaw,
    ArabicLetterAlefMaksura,
    ArabicLetterYeh,
    ArabicFathatan,
    ArabicDammatan,
    ArabicKasratan,
    ArabicFatha,
    ArabicDamma,
    ArabicKasra,
    ArabicShadda,
    ArabicSukun,
}

impl ArabicCharacter {
    /// Returns the underlying Unicode character.
    fn character(&self) -> Box<dyn UnicodeCharacter> {
        match self {
            ArabicLetterHamza => Box::new(crate::core::characters::ArabicLetterHamza::new()),
            ArabicLetterAlefWithMaddaAbove => {
                Box::new(crate::core::characters::ArabicLetterAlefWithMaddaAbove::new())
            }
            ArabicLetterAlefWithHamzaAbove => {
                Box::new(crate::core::characters::ArabicLetterAlefWithHamzaAbove::new())
            }
            ArabicLetterWawWithHamzaAbove => {
                Box::new(crate::core::characters::ArabicLetterWawWithHamzaAbove::new())
            }
            ArabicLetterAlefWithHamzaBelow => {
                Box::new(crate::core::characters::ArabicLetterAlefWithHamzaBelow::new())
            }
            ArabicLetterYehWithHamzaAbove => {
                Box::new(crate::core::characters::ArabicLetterYehWithHamzaAbove::new())
            }
            ArabicLetterAlef => Box::new(crate::core::characters::ArabicLetterAlef::new()),
            ArabicLetterBeh => Box::new(crate::core::characters::ArabicLetterBeh::new()),
            ArabicLetterTehMarbuta => {
                Box::new(crate::core::characters::ArabicLetterTehMarbuta::new())
            }
            ArabicLetterTeh => Box::new(crate::core::characters::ArabicLetterTeh::new()),
            ArabicLetterTheh => Box::new(crate::core::characters::ArabicLetterTheh::new()),
            ArabicLetterJeem => Box::new(crate::core::characters::ArabicLetterJeem::new()),
            ArabicLetterHah => Box::new(crate::core::characters::ArabicLetterHah::new()),
            ArabicLetterKhah => Box::new(crate::core::characters::ArabicLetterKhah::new()),
            ArabicLetterDal => Box::new(crate::core::characters::ArabicLetterDal::new()),
            ArabicLetterThal => Box::new(crate::core::characters::ArabicLetterThal::new()),
            ArabicLetterReh => Box::new(crate::core::characters::ArabicLetterReh::new()),
            ArabicLetterZain => Box::new(crate::core::characters::ArabicLetterZain::new()),
            ArabicLetterSeen => Box::new(crate::core::characters::ArabicLetterSeen::new()),
            ArabicLetterSheen => Box::new(crate::core::characters::ArabicLetterSheen::new()),
            ArabicLetterSad => Box::new(crate::core::characters::ArabicLetterSad::new()),
            ArabicLetterDad => Box::new(crate::core::characters::ArabicLetterDad::new()),
            ArabicLetterTah => Box::new(crate::core::characters::ArabicLetterTah::new()),
            ArabicLetterZah => Box::new(crate::core::characters::ArabicLetterZah::new()),
            ArabicLetterAin => Box::new(crate::core::characters::ArabicLetterAin::new()),
            ArabicLetterGhain => Box::new(crate::core::characters::ArabicLetterGhain::new()),
            ArabicTatweel => Box::new(crate::core::characters::ArabicTatweel::new()),
            ArabicLetterFeh => Box::new(crate::core::characters::ArabicLetterFeh::new()),
            ArabicLetterQaf => Box::new(crate::core::characters::ArabicLetterQaf::new()),
            ArabicLetterKaf => Box::new(crate::core::characters::ArabicLetterKaf::new()),
            ArabicLetterLam => Box::new(crate::core::characters::ArabicLetterLam::new()),
            ArabicLetterMeem => Box::new(crate::core::characters::ArabicLetterMeem::new()),
            ArabicLetterNoon => Box::new(crate::core::characters::ArabicLetterNoon::new()),
            ArabicLetterHeh => Box::new(crate::core::characters::ArabicLetterHeh::new()),
            ArabicLetterWaw => Box::new(crate::core::characters::ArabicLetterWaw::new()),
            ArabicLetterAlefMaksura => {
                Box::new(crate::core::characters::ArabicLetterAlefMaksura::new())
            }
            ArabicLetterYeh => Box::new(crate::core::characters::ArabicLetterYeh::new()),
            ArabicFathatan => Box::new(crate::core::characters::ArabicFathatan::new()),
            ArabicDammatan => Box::new(crate::core::characters::ArabicDammatan::new()),
            ArabicKasratan => Box::new(crate::core::characters::ArabicKasratan::new()),
            ArabicFatha => Box::new(crate::core::characters::ArabicFatha::new()),
            ArabicDamma => Box::new(crate::core::characters::ArabicDamma::new()),
            ArabicKasra => Box::new(crate::core::characters::ArabicKasra::new()),
            ArabicShadda => Box::new(crate::core::characters::ArabicShadda::new()),
            ArabicSukun => Box::new(crate::core::characters::ArabicSukun::new()),
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
            '\u{0621}' => Ok(ArabicLetterHamza),
            '\u{0622}' => Ok(ArabicLetterAlefWithMaddaAbove),
            '\u{0623}' => Ok(ArabicLetterAlefWithHamzaAbove),
            '\u{0624}' => Ok(ArabicLetterWawWithHamzaAbove),
            '\u{0625}' => Ok(ArabicLetterAlefWithHamzaBelow),
            '\u{0626}' => Ok(ArabicLetterYehWithHamzaAbove),
            '\u{0627}' => Ok(ArabicLetterAlef),
            '\u{0628}' => Ok(ArabicLetterBeh),
            '\u{0629}' => Ok(ArabicLetterTehMarbuta),
            '\u{062A}' => Ok(ArabicLetterTeh),
            '\u{062B}' => Ok(ArabicLetterTheh),
            '\u{062C}' => Ok(ArabicLetterJeem),
            '\u{062D}' => Ok(ArabicLetterHah),
            '\u{062E}' => Ok(ArabicLetterKhah),
            '\u{062F}' => Ok(ArabicLetterDal),
            '\u{0630}' => Ok(ArabicLetterThal),
            '\u{0631}' => Ok(ArabicLetterReh),
            '\u{0632}' => Ok(ArabicLetterZain),
            '\u{0633}' => Ok(ArabicLetterSeen),
            '\u{0634}' => Ok(ArabicLetterSheen),
            '\u{0635}' => Ok(ArabicLetterSad),
            '\u{0636}' => Ok(ArabicLetterDad),
            '\u{0637}' => Ok(ArabicLetterTah),
            '\u{0638}' => Ok(ArabicLetterZah),
            '\u{0639}' => Ok(ArabicLetterAin),
            '\u{063A}' => Ok(ArabicLetterGhain),
            '\u{0640}' => Ok(ArabicTatweel),
            '\u{0641}' => Ok(ArabicLetterFeh),
            '\u{0642}' => Ok(ArabicLetterQaf),
            '\u{0643}' => Ok(ArabicLetterKaf),
            '\u{0644}' => Ok(ArabicLetterLam),
            '\u{0645}' => Ok(ArabicLetterMeem),
            '\u{0646}' => Ok(ArabicLetterNoon),
            '\u{0647}' => Ok(ArabicLetterHeh),
            '\u{0648}' => Ok(ArabicLetterWaw),
            '\u{0649}' => Ok(ArabicLetterAlefMaksura),
            '\u{064A}' => Ok(ArabicLetterYeh),
            '\u{064B}' => Ok(ArabicFathatan),
            '\u{064C}' => Ok(ArabicDammatan),
            '\u{064D}' => Ok(ArabicKasratan),
            '\u{064E}' => Ok(ArabicFatha),
            '\u{064F}' => Ok(ArabicDamma),
            '\u{0650}' => Ok(ArabicKasra),
            '\u{0651}' => Ok(ArabicShadda),
            '\u{0652}' => Ok(ArabicSukun),
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

    use super::ArabicDamma;
    use super::ArabicDammatan;
    use super::ArabicFatha;
    use super::ArabicFathatan;
    use super::ArabicKasra;
    use super::ArabicKasratan;
    use super::ArabicLetterAin;
    use super::ArabicLetterAlef;
    use super::ArabicLetterAlefMaksura;
    use super::ArabicLetterAlefWithHamzaAbove;
    use super::ArabicLetterAlefWithHamzaBelow;
    use super::ArabicLetterAlefWithMaddaAbove;
    use super::ArabicLetterBeh;
    use super::ArabicLetterDad;
    use super::ArabicLetterDal;
    use super::ArabicLetterFeh;
    use super::ArabicLetterGhain;
    use super::ArabicLetterHah;
    use super::ArabicLetterHamza;
    use super::ArabicLetterHeh;
    use super::ArabicLetterJeem;
    use super::ArabicLetterKaf;
    use super::ArabicLetterKhah;
    use super::ArabicLetterLam;
    use super::ArabicLetterMeem;
    use super::ArabicLetterNoon;
    use super::ArabicLetterQaf;
    use super::ArabicLetterReh;
    use super::ArabicLetterSad;
    use super::ArabicLetterSeen;
    use super::ArabicLetterSheen;
    use super::ArabicLetterTah;
    use super::ArabicLetterTeh;
    use super::ArabicLetterTehMarbuta;
    use super::ArabicLetterThal;
    use super::ArabicLetterTheh;
    use super::ArabicLetterWaw;
    use super::ArabicLetterWawWithHamzaAbove;
    use super::ArabicLetterYeh;
    use super::ArabicLetterYehWithHamzaAbove;
    use super::ArabicLetterZah;
    use super::ArabicLetterZain;
    use super::ArabicShadda;
    use super::ArabicSukun;
    use super::ArabicTatweel;

    #[test]
    #[should_panic]
    fn accepts_only_arabic_script_characters() {
        ArabicCharacter::try_from('a').unwrap();
    }

    #[test]
    fn arabic_letter_hamza() {
        assert_eq!(ArabicLetterHamza.block(), "Arabic");
        assert_eq!(ArabicLetterHamza.name(), "Arabic Letter Hamza");
        assert_eq!(ArabicLetterHamza.scalar_value(), '\u{0621}');

        assert_eq!(
            format!("{:?}", ArabicLetterHamza),
            "Arabic Letter Hamza { \u{0621} }"
        );
        assert_eq!(format!("{}", ArabicLetterHamza), "\u{0621}");
        assert_eq!(ArabicLetterHamza, ArabicLetterHamza);
        assert_eq!(ArabicLetterHamza, '\u{0621}');
        assert_eq!(ArabicLetterHamza, "\u{0621}");
        assert_eq!(ArabicLetterHamza, "\u{0621}".to_string());
        assert_eq!(
            ArabicLetterHamza,
            ArabicCharacter::try_from('\u{0621}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_alef_with_madda_above() {
        assert_eq!(ArabicLetterAlefWithMaddaAbove.block(), "Arabic");
        assert_eq!(
            ArabicLetterAlefWithMaddaAbove.name(),
            "Arabic Letter Alef With Madda Above"
        );
        assert_eq!(ArabicLetterAlefWithMaddaAbove.scalar_value(), '\u{0622}');

        assert_eq!(
            format!("{:?}", ArabicLetterAlefWithMaddaAbove),
            "Arabic Letter Alef With Madda Above { \u{0622} }"
        );
        assert_eq!(format!("{}", ArabicLetterAlefWithMaddaAbove), "\u{0622}");
        assert_eq!(
            ArabicLetterAlefWithMaddaAbove,
            ArabicLetterAlefWithMaddaAbove
        );
        assert_eq!(ArabicLetterAlefWithMaddaAbove, '\u{0622}');
        assert_eq!(ArabicLetterAlefWithMaddaAbove, "\u{0622}");
        assert_eq!(ArabicLetterAlefWithMaddaAbove, "\u{0622}".to_string());
        assert_eq!(
            ArabicLetterAlefWithMaddaAbove,
            ArabicCharacter::try_from('\u{0622}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_alef_with_hamza_above() {
        assert_eq!(ArabicLetterAlefWithHamzaAbove.block(), "Arabic");
        assert_eq!(
            ArabicLetterAlefWithHamzaAbove.name(),
            "Arabic Letter Alef With Hamza Above"
        );
        assert_eq!(ArabicLetterAlefWithHamzaAbove.scalar_value(), '\u{0623}');

        assert_eq!(
            format!("{:?}", ArabicLetterAlefWithHamzaAbove),
            "Arabic Letter Alef With Hamza Above { \u{0623} }"
        );
        assert_eq!(format!("{}", ArabicLetterAlefWithHamzaAbove), "\u{0623}");
        assert_eq!(
            ArabicLetterAlefWithHamzaAbove,
            ArabicLetterAlefWithHamzaAbove
        );
        assert_eq!(ArabicLetterAlefWithHamzaAbove, '\u{0623}');
        assert_eq!(ArabicLetterAlefWithHamzaAbove, "\u{0623}");
        assert_eq!(ArabicLetterAlefWithHamzaAbove, "\u{0623}".to_string());
        assert_eq!(
            ArabicLetterAlefWithHamzaAbove,
            ArabicCharacter::try_from('\u{0623}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_waw_with_hamza_above() {
        assert_eq!(ArabicLetterWawWithHamzaAbove.block(), "Arabic");
        assert_eq!(
            ArabicLetterWawWithHamzaAbove.name(),
            "Arabic Letter Waw With Hamza Above"
        );
        assert_eq!(ArabicLetterWawWithHamzaAbove.scalar_value(), '\u{0624}');

        assert_eq!(
            format!("{:?}", ArabicLetterWawWithHamzaAbove),
            "Arabic Letter Waw With Hamza Above { \u{0624} }"
        );
        assert_eq!(format!("{}", ArabicLetterWawWithHamzaAbove), "\u{0624}");
        assert_eq!(ArabicLetterWawWithHamzaAbove, ArabicLetterWawWithHamzaAbove);
        assert_eq!(ArabicLetterWawWithHamzaAbove, '\u{0624}');
        assert_eq!(ArabicLetterWawWithHamzaAbove, "\u{0624}");
        assert_eq!(ArabicLetterWawWithHamzaAbove, "\u{0624}".to_string());
        assert_eq!(
            ArabicLetterWawWithHamzaAbove,
            ArabicCharacter::try_from('\u{0624}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_alef_with_hamza_below() {
        assert_eq!(ArabicLetterAlefWithHamzaBelow.block(), "Arabic");
        assert_eq!(
            ArabicLetterAlefWithHamzaBelow.name(),
            "Arabic Letter Alef With Hamza Below"
        );
        assert_eq!(ArabicLetterAlefWithHamzaBelow.scalar_value(), '\u{0625}');

        assert_eq!(
            format!("{:?}", ArabicLetterAlefWithHamzaBelow),
            "Arabic Letter Alef With Hamza Below { \u{0625} }"
        );
        assert_eq!(format!("{}", ArabicLetterAlefWithHamzaBelow), "\u{0625}");
        assert_eq!(
            ArabicLetterAlefWithHamzaBelow,
            ArabicLetterAlefWithHamzaBelow
        );
        assert_eq!(ArabicLetterAlefWithHamzaBelow, '\u{0625}');
        assert_eq!(ArabicLetterAlefWithHamzaBelow, "\u{0625}");
        assert_eq!(ArabicLetterAlefWithHamzaBelow, "\u{0625}".to_string());
        assert_eq!(
            ArabicLetterAlefWithHamzaBelow,
            ArabicCharacter::try_from('\u{0625}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_yeh_with_hamza_above() {
        assert_eq!(ArabicLetterYehWithHamzaAbove.block(), "Arabic");
        assert_eq!(
            ArabicLetterYehWithHamzaAbove.name(),
            "Arabic Letter Yeh With Hamza Above"
        );
        assert_eq!(ArabicLetterYehWithHamzaAbove.scalar_value(), '\u{0626}');

        assert_eq!(
            format!("{:?}", ArabicLetterYehWithHamzaAbove),
            "Arabic Letter Yeh With Hamza Above { \u{0626} }"
        );
        assert_eq!(format!("{}", ArabicLetterYehWithHamzaAbove), "\u{0626}");
        assert_eq!(ArabicLetterYehWithHamzaAbove, ArabicLetterYehWithHamzaAbove);
        assert_eq!(ArabicLetterYehWithHamzaAbove, '\u{0626}');
        assert_eq!(ArabicLetterYehWithHamzaAbove, "\u{0626}");
        assert_eq!(ArabicLetterYehWithHamzaAbove, "\u{0626}".to_string());
        assert_eq!(
            ArabicLetterYehWithHamzaAbove,
            ArabicCharacter::try_from('\u{0626}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_alef() {
        assert_eq!(ArabicLetterAlef.block(), "Arabic");
        assert_eq!(ArabicLetterAlef.name(), "Arabic Letter Alef");
        assert_eq!(ArabicLetterAlef.scalar_value(), '\u{0627}');

        assert_eq!(
            format!("{:?}", ArabicLetterAlef),
            "Arabic Letter Alef { \u{0627} }"
        );
        assert_eq!(format!("{}", ArabicLetterAlef), "\u{0627}");
        assert_eq!(ArabicLetterAlef, ArabicLetterAlef);
        assert_eq!(ArabicLetterAlef, '\u{0627}');
        assert_eq!(ArabicLetterAlef, "\u{0627}");
        assert_eq!(ArabicLetterAlef, "\u{0627}".to_string());
        assert_eq!(
            ArabicLetterAlef,
            ArabicCharacter::try_from('\u{0627}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_beh() {
        assert_eq!(ArabicLetterBeh.block(), "Arabic");
        assert_eq!(ArabicLetterBeh.name(), "Arabic Letter Beh");
        assert_eq!(ArabicLetterBeh.scalar_value(), '\u{0628}');

        assert_eq!(
            format!("{:?}", ArabicLetterBeh),
            "Arabic Letter Beh { \u{0628} }"
        );
        assert_eq!(format!("{}", ArabicLetterBeh), "\u{0628}");
        assert_eq!(ArabicLetterBeh, ArabicLetterBeh);
        assert_eq!(ArabicLetterBeh, '\u{0628}');
        assert_eq!(ArabicLetterBeh, "\u{0628}");
        assert_eq!(ArabicLetterBeh, "\u{0628}".to_string());
        assert_eq!(
            ArabicLetterBeh,
            ArabicCharacter::try_from('\u{0628}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_teh_marbuta() {
        assert_eq!(ArabicLetterTehMarbuta.block(), "Arabic");
        assert_eq!(ArabicLetterTehMarbuta.name(), "Arabic Letter Teh Marbuta");
        assert_eq!(ArabicLetterTehMarbuta.scalar_value(), '\u{0629}');

        assert_eq!(
            format!("{:?}", ArabicLetterTehMarbuta),
            "Arabic Letter Teh Marbuta { \u{0629} }"
        );
        assert_eq!(format!("{}", ArabicLetterTehMarbuta), "\u{0629}");
        assert_eq!(ArabicLetterTehMarbuta, ArabicLetterTehMarbuta);
        assert_eq!(ArabicLetterTehMarbuta, '\u{0629}');
        assert_eq!(ArabicLetterTehMarbuta, "\u{0629}");
        assert_eq!(ArabicLetterTehMarbuta, "\u{0629}".to_string());
        assert_eq!(
            ArabicLetterTehMarbuta,
            ArabicCharacter::try_from('\u{0629}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_teh() {
        assert_eq!(ArabicLetterTeh.block(), "Arabic");
        assert_eq!(ArabicLetterTeh.name(), "Arabic Letter Teh");
        assert_eq!(ArabicLetterTeh.scalar_value(), '\u{062A}');

        assert_eq!(
            format!("{:?}", ArabicLetterTeh),
            "Arabic Letter Teh { \u{062A} }"
        );
        assert_eq!(format!("{}", ArabicLetterTeh), "\u{062A}");
        assert_eq!(ArabicLetterTeh, ArabicLetterTeh);
        assert_eq!(ArabicLetterTeh, '\u{062A}');
        assert_eq!(ArabicLetterTeh, "\u{062A}");
        assert_eq!(ArabicLetterTeh, "\u{062A}".to_string());
        assert_eq!(
            ArabicLetterTeh,
            ArabicCharacter::try_from('\u{062A}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_theh() {
        assert_eq!(ArabicLetterTheh.block(), "Arabic");
        assert_eq!(ArabicLetterTheh.name(), "Arabic Letter Theh");
        assert_eq!(ArabicLetterTheh.scalar_value(), '\u{062B}');

        assert_eq!(
            format!("{:?}", ArabicLetterTheh),
            "Arabic Letter Theh { \u{062B} }"
        );
        assert_eq!(format!("{}", ArabicLetterTheh), "\u{062B}");
        assert_eq!(ArabicLetterTheh, ArabicLetterTheh);
        assert_eq!(ArabicLetterTheh, '\u{062B}');
        assert_eq!(ArabicLetterTheh, "\u{062B}");
        assert_eq!(ArabicLetterTheh, "\u{062B}".to_string());
        assert_eq!(
            ArabicLetterTheh,
            ArabicCharacter::try_from('\u{062B}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_jeem() {
        assert_eq!(ArabicLetterJeem.block(), "Arabic");
        assert_eq!(ArabicLetterJeem.name(), "Arabic Letter Jeem");
        assert_eq!(ArabicLetterJeem.scalar_value(), '\u{062C}');

        assert_eq!(
            format!("{:?}", ArabicLetterJeem),
            "Arabic Letter Jeem { \u{062C} }"
        );
        assert_eq!(format!("{}", ArabicLetterJeem), "\u{062C}");
        assert_eq!(ArabicLetterJeem, ArabicLetterJeem);
        assert_eq!(ArabicLetterJeem, '\u{062C}');
        assert_eq!(ArabicLetterJeem, "\u{062C}");
        assert_eq!(ArabicLetterJeem, "\u{062C}".to_string());
        assert_eq!(
            ArabicLetterJeem,
            ArabicCharacter::try_from('\u{062C}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_hah() {
        assert_eq!(ArabicLetterHah.block(), "Arabic");
        assert_eq!(ArabicLetterHah.name(), "Arabic Letter Hah");
        assert_eq!(ArabicLetterHah.scalar_value(), '\u{062D}');

        assert_eq!(
            format!("{:?}", ArabicLetterHah),
            "Arabic Letter Hah { \u{062D} }"
        );
        assert_eq!(format!("{}", ArabicLetterHah), "\u{062D}");
        assert_eq!(ArabicLetterHah, ArabicLetterHah);
        assert_eq!(ArabicLetterHah, '\u{062D}');
        assert_eq!(ArabicLetterHah, "\u{062D}");
        assert_eq!(ArabicLetterHah, "\u{062D}".to_string());
        assert_eq!(
            ArabicLetterHah,
            ArabicCharacter::try_from('\u{062D}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_khah() {
        assert_eq!(ArabicLetterKhah.block(), "Arabic");
        assert_eq!(ArabicLetterKhah.name(), "Arabic Letter Khah");
        assert_eq!(ArabicLetterKhah.scalar_value(), '\u{062E}');

        assert_eq!(
            format!("{:?}", ArabicLetterKhah),
            "Arabic Letter Khah { \u{062E} }"
        );
        assert_eq!(format!("{}", ArabicLetterKhah), "\u{062E}");
        assert_eq!(ArabicLetterKhah, ArabicLetterKhah);
        assert_eq!(ArabicLetterKhah, '\u{062E}');
        assert_eq!(ArabicLetterKhah, "\u{062E}");
        assert_eq!(ArabicLetterKhah, "\u{062E}".to_string());
        assert_eq!(
            ArabicLetterKhah,
            ArabicCharacter::try_from('\u{062E}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_dal() {
        assert_eq!(ArabicLetterDal.block(), "Arabic");
        assert_eq!(ArabicLetterDal.name(), "Arabic Letter Dal");
        assert_eq!(ArabicLetterDal.scalar_value(), '\u{062F}');

        assert_eq!(
            format!("{:?}", ArabicLetterDal),
            "Arabic Letter Dal { \u{062F} }"
        );
        assert_eq!(format!("{}", ArabicLetterDal), "\u{062F}");
        assert_eq!(ArabicLetterDal, ArabicLetterDal);
        assert_eq!(ArabicLetterDal, '\u{062F}');
        assert_eq!(ArabicLetterDal, "\u{062F}");
        assert_eq!(ArabicLetterDal, "\u{062F}".to_string());
        assert_eq!(
            ArabicLetterDal,
            ArabicCharacter::try_from('\u{062F}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_thal() {
        assert_eq!(ArabicLetterThal.block(), "Arabic");
        assert_eq!(ArabicLetterThal.name(), "Arabic Letter Thal");
        assert_eq!(ArabicLetterThal.scalar_value(), '\u{0630}');

        assert_eq!(
            format!("{:?}", ArabicLetterThal),
            "Arabic Letter Thal { \u{0630} }"
        );
        assert_eq!(format!("{}", ArabicLetterThal), "\u{0630}");
        assert_eq!(ArabicLetterThal, ArabicLetterThal);
        assert_eq!(ArabicLetterThal, '\u{0630}');
        assert_eq!(ArabicLetterThal, "\u{0630}");
        assert_eq!(ArabicLetterThal, "\u{0630}".to_string());
        assert_eq!(
            ArabicLetterThal,
            ArabicCharacter::try_from('\u{0630}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_reh() {
        assert_eq!(ArabicLetterReh.block(), "Arabic");
        assert_eq!(ArabicLetterReh.name(), "Arabic Letter Reh");
        assert_eq!(ArabicLetterReh.scalar_value(), '\u{0631}');

        assert_eq!(
            format!("{:?}", ArabicLetterReh),
            "Arabic Letter Reh { \u{0631} }"
        );
        assert_eq!(format!("{}", ArabicLetterReh), "\u{0631}");
        assert_eq!(ArabicLetterReh, ArabicLetterReh);
        assert_eq!(ArabicLetterReh, '\u{0631}');
        assert_eq!(ArabicLetterReh, "\u{0631}");
        assert_eq!(ArabicLetterReh, "\u{0631}".to_string());
        assert_eq!(
            ArabicLetterReh,
            ArabicCharacter::try_from('\u{0631}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_zain() {
        assert_eq!(ArabicLetterZain.block(), "Arabic");
        assert_eq!(ArabicLetterZain.name(), "Arabic Letter Zain");
        assert_eq!(ArabicLetterZain.scalar_value(), '\u{0632}');

        assert_eq!(
            format!("{:?}", ArabicLetterZain),
            "Arabic Letter Zain { \u{0632} }"
        );
        assert_eq!(format!("{}", ArabicLetterZain), "\u{0632}");
        assert_eq!(ArabicLetterZain, ArabicLetterZain);
        assert_eq!(ArabicLetterZain, '\u{0632}');
        assert_eq!(ArabicLetterZain, "\u{0632}");
        assert_eq!(ArabicLetterZain, "\u{0632}".to_string());
        assert_eq!(
            ArabicLetterZain,
            ArabicCharacter::try_from('\u{0632}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_seen() {
        assert_eq!(ArabicLetterSeen.block(), "Arabic");
        assert_eq!(ArabicLetterSeen.name(), "Arabic Letter Seen");
        assert_eq!(ArabicLetterSeen.scalar_value(), '\u{0633}');

        assert_eq!(
            format!("{:?}", ArabicLetterSeen),
            "Arabic Letter Seen { \u{0633} }"
        );
        assert_eq!(format!("{}", ArabicLetterSeen), "\u{0633}");
        assert_eq!(ArabicLetterSeen, ArabicLetterSeen);
        assert_eq!(ArabicLetterSeen, '\u{0633}');
        assert_eq!(ArabicLetterSeen, "\u{0633}");
        assert_eq!(ArabicLetterSeen, "\u{0633}".to_string());
        assert_eq!(
            ArabicLetterSeen,
            ArabicCharacter::try_from('\u{0633}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_sheen() {
        assert_eq!(ArabicLetterSheen.block(), "Arabic");
        assert_eq!(ArabicLetterSheen.name(), "Arabic Letter Sheen");
        assert_eq!(ArabicLetterSheen.scalar_value(), '\u{0634}');

        assert_eq!(
            format!("{:?}", ArabicLetterSheen),
            "Arabic Letter Sheen { \u{0634} }"
        );
        assert_eq!(format!("{}", ArabicLetterSheen), "\u{0634}");
        assert_eq!(ArabicLetterSheen, ArabicLetterSheen);
        assert_eq!(ArabicLetterSheen, '\u{0634}');
        assert_eq!(ArabicLetterSheen, "\u{0634}");
        assert_eq!(ArabicLetterSheen, "\u{0634}".to_string());
        assert_eq!(
            ArabicLetterSheen,
            ArabicCharacter::try_from('\u{0634}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_sad() {
        assert_eq!(ArabicLetterSad.block(), "Arabic");
        assert_eq!(ArabicLetterSad.name(), "Arabic Letter Sad");
        assert_eq!(ArabicLetterSad.scalar_value(), '\u{0635}');

        assert_eq!(
            format!("{:?}", ArabicLetterSad),
            "Arabic Letter Sad { \u{0635} }"
        );
        assert_eq!(format!("{}", ArabicLetterSad), "\u{0635}");
        assert_eq!(ArabicLetterSad, ArabicLetterSad);
        assert_eq!(ArabicLetterSad, '\u{0635}');
        assert_eq!(ArabicLetterSad, "\u{0635}");
        assert_eq!(ArabicLetterSad, "\u{0635}".to_string());
        assert_eq!(
            ArabicLetterSad,
            ArabicCharacter::try_from('\u{0635}').unwrap()
        );
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

    #[test]
    fn arabic_letter_tah() {
        assert_eq!(ArabicLetterTah.block(), "Arabic");
        assert_eq!(ArabicLetterTah.name(), "Arabic Letter Tah");
        assert_eq!(ArabicLetterTah.scalar_value(), '\u{0637}');

        assert_eq!(
            format!("{:?}", ArabicLetterTah),
            "Arabic Letter Tah { \u{0637} }"
        );
        assert_eq!(format!("{}", ArabicLetterTah), "\u{0637}");
        assert_eq!(ArabicLetterTah, ArabicLetterTah);
        assert_eq!(ArabicLetterTah, '\u{0637}');
        assert_eq!(ArabicLetterTah, "\u{0637}");
        assert_eq!(ArabicLetterTah, "\u{0637}".to_string());
        assert_eq!(
            ArabicLetterTah,
            ArabicCharacter::try_from('\u{0637}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_zah() {
        assert_eq!(ArabicLetterZah.block(), "Arabic");
        assert_eq!(ArabicLetterZah.name(), "Arabic Letter Zah");
        assert_eq!(ArabicLetterZah.scalar_value(), '\u{0638}');

        assert_eq!(
            format!("{:?}", ArabicLetterZah),
            "Arabic Letter Zah { \u{0638} }"
        );
        assert_eq!(format!("{}", ArabicLetterZah), "\u{0638}");
        assert_eq!(ArabicLetterZah, ArabicLetterZah);
        assert_eq!(ArabicLetterZah, '\u{0638}');
        assert_eq!(ArabicLetterZah, "\u{0638}");
        assert_eq!(ArabicLetterZah, "\u{0638}".to_string());
        assert_eq!(
            ArabicLetterZah,
            ArabicCharacter::try_from('\u{0638}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_ain() {
        assert_eq!(ArabicLetterAin.block(), "Arabic");
        assert_eq!(ArabicLetterAin.name(), "Arabic Letter Ain");
        assert_eq!(ArabicLetterAin.scalar_value(), '\u{0639}');

        assert_eq!(
            format!("{:?}", ArabicLetterAin),
            "Arabic Letter Ain { \u{0639} }"
        );
        assert_eq!(format!("{}", ArabicLetterAin), "\u{0639}");
        assert_eq!(ArabicLetterAin, ArabicLetterAin);
        assert_eq!(ArabicLetterAin, '\u{0639}');
        assert_eq!(ArabicLetterAin, "\u{0639}");
        assert_eq!(ArabicLetterAin, "\u{0639}".to_string());
        assert_eq!(
            ArabicLetterAin,
            ArabicCharacter::try_from('\u{0639}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_ghain() {
        assert_eq!(ArabicLetterGhain.block(), "Arabic");
        assert_eq!(ArabicLetterGhain.name(), "Arabic Letter Ghain");
        assert_eq!(ArabicLetterGhain.scalar_value(), '\u{063A}');

        assert_eq!(
            format!("{:?}", ArabicLetterGhain),
            "Arabic Letter Ghain { \u{063A} }"
        );
        assert_eq!(format!("{}", ArabicLetterGhain), "\u{063A}");
        assert_eq!(ArabicLetterGhain, ArabicLetterGhain);
        assert_eq!(ArabicLetterGhain, '\u{063A}');
        assert_eq!(ArabicLetterGhain, "\u{063A}");
        assert_eq!(ArabicLetterGhain, "\u{063A}".to_string());
        assert_eq!(
            ArabicLetterGhain,
            ArabicCharacter::try_from('\u{063A}').unwrap()
        );
    }

    #[test]
    fn arabic_tatweel() {
        assert_eq!(ArabicTatweel.block(), "Arabic");
        assert_eq!(ArabicTatweel.name(), "Arabic Tatweel");
        assert_eq!(ArabicTatweel.scalar_value(), '\u{0640}');

        assert_eq!(
            format!("{:?}", ArabicTatweel),
            "Arabic Tatweel { \u{0640} }"
        );
        assert_eq!(format!("{}", ArabicTatweel), "\u{0640}");
        assert_eq!(ArabicTatweel, ArabicTatweel);
        assert_eq!(ArabicTatweel, '\u{0640}');
        assert_eq!(ArabicTatweel, "\u{0640}");
        assert_eq!(ArabicTatweel, "\u{0640}".to_string());
        assert_eq!(
            ArabicTatweel,
            ArabicCharacter::try_from('\u{0640}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_feh() {
        assert_eq!(ArabicLetterFeh.block(), "Arabic");
        assert_eq!(ArabicLetterFeh.name(), "Arabic Letter Feh");
        assert_eq!(ArabicLetterFeh.scalar_value(), '\u{0641}');

        assert_eq!(
            format!("{:?}", ArabicLetterFeh),
            "Arabic Letter Feh { \u{0641} }"
        );
        assert_eq!(format!("{}", ArabicLetterFeh), "\u{0641}");
        assert_eq!(ArabicLetterFeh, ArabicLetterFeh);
        assert_eq!(ArabicLetterFeh, '\u{0641}');
        assert_eq!(ArabicLetterFeh, "\u{0641}");
        assert_eq!(ArabicLetterFeh, "\u{0641}".to_string());
        assert_eq!(
            ArabicLetterFeh,
            ArabicCharacter::try_from('\u{0641}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_qaf() {
        assert_eq!(ArabicLetterQaf.block(), "Arabic");
        assert_eq!(ArabicLetterQaf.name(), "Arabic Letter Qaf");
        assert_eq!(ArabicLetterQaf.scalar_value(), '\u{0642}');

        assert_eq!(
            format!("{:?}", ArabicLetterQaf),
            "Arabic Letter Qaf { \u{0642} }"
        );
        assert_eq!(format!("{}", ArabicLetterQaf), "\u{0642}");
        assert_eq!(ArabicLetterQaf, ArabicLetterQaf);
        assert_eq!(ArabicLetterQaf, '\u{0642}');
        assert_eq!(ArabicLetterQaf, "\u{0642}");
        assert_eq!(ArabicLetterQaf, "\u{0642}".to_string());
        assert_eq!(
            ArabicLetterQaf,
            ArabicCharacter::try_from('\u{0642}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_kaf() {
        assert_eq!(ArabicLetterKaf.block(), "Arabic");
        assert_eq!(ArabicLetterKaf.name(), "Arabic Letter Kaf");
        assert_eq!(ArabicLetterKaf.scalar_value(), '\u{0643}');

        assert_eq!(
            format!("{:?}", ArabicLetterKaf),
            "Arabic Letter Kaf { \u{0643} }"
        );
        assert_eq!(format!("{}", ArabicLetterKaf), "\u{0643}");
        assert_eq!(ArabicLetterKaf, ArabicLetterKaf);
        assert_eq!(ArabicLetterKaf, '\u{0643}');
        assert_eq!(ArabicLetterKaf, "\u{0643}");
        assert_eq!(ArabicLetterKaf, "\u{0643}".to_string());
        assert_eq!(
            ArabicLetterKaf,
            ArabicCharacter::try_from('\u{0643}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_lam() {
        assert_eq!(ArabicLetterLam.block(), "Arabic");
        assert_eq!(ArabicLetterLam.name(), "Arabic Letter Lam");
        assert_eq!(ArabicLetterLam.scalar_value(), '\u{0644}');

        assert_eq!(
            format!("{:?}", ArabicLetterLam),
            "Arabic Letter Lam { \u{0644} }"
        );
        assert_eq!(format!("{}", ArabicLetterLam), "\u{0644}");
        assert_eq!(ArabicLetterLam, ArabicLetterLam);
        assert_eq!(ArabicLetterLam, '\u{0644}');
        assert_eq!(ArabicLetterLam, "\u{0644}");
        assert_eq!(ArabicLetterLam, "\u{0644}".to_string());
        assert_eq!(
            ArabicLetterLam,
            ArabicCharacter::try_from('\u{0644}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_meem() {
        assert_eq!(ArabicLetterMeem.block(), "Arabic");
        assert_eq!(ArabicLetterMeem.name(), "Arabic Letter Meem");
        assert_eq!(ArabicLetterMeem.scalar_value(), '\u{0645}');

        assert_eq!(
            format!("{:?}", ArabicLetterMeem),
            "Arabic Letter Meem { \u{0645} }"
        );
        assert_eq!(format!("{}", ArabicLetterMeem), "\u{0645}");
        assert_eq!(ArabicLetterMeem, ArabicLetterMeem);
        assert_eq!(ArabicLetterMeem, '\u{0645}');
        assert_eq!(ArabicLetterMeem, "\u{0645}");
        assert_eq!(ArabicLetterMeem, "\u{0645}".to_string());
        assert_eq!(
            ArabicLetterMeem,
            ArabicCharacter::try_from('\u{0645}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_noon() {
        assert_eq!(ArabicLetterNoon.block(), "Arabic");
        assert_eq!(ArabicLetterNoon.name(), "Arabic Letter Noon");
        assert_eq!(ArabicLetterNoon.scalar_value(), '\u{0646}');

        assert_eq!(
            format!("{:?}", ArabicLetterNoon),
            "Arabic Letter Noon { \u{0646} }"
        );
        assert_eq!(format!("{}", ArabicLetterNoon), "\u{0646}");
        assert_eq!(ArabicLetterNoon, ArabicLetterNoon);
        assert_eq!(ArabicLetterNoon, '\u{0646}');
        assert_eq!(ArabicLetterNoon, "\u{0646}");
        assert_eq!(ArabicLetterNoon, "\u{0646}".to_string());
        assert_eq!(
            ArabicLetterNoon,
            ArabicCharacter::try_from('\u{0646}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_heh() {
        assert_eq!(ArabicLetterHeh.block(), "Arabic");
        assert_eq!(ArabicLetterHeh.name(), "Arabic Letter Heh");
        assert_eq!(ArabicLetterHeh.scalar_value(), '\u{0647}');

        assert_eq!(
            format!("{:?}", ArabicLetterHeh),
            "Arabic Letter Heh { \u{0647} }"
        );
        assert_eq!(format!("{}", ArabicLetterHeh), "\u{0647}");
        assert_eq!(ArabicLetterHeh, ArabicLetterHeh);
        assert_eq!(ArabicLetterHeh, '\u{0647}');
        assert_eq!(ArabicLetterHeh, "\u{0647}");
        assert_eq!(ArabicLetterHeh, "\u{0647}".to_string());
        assert_eq!(
            ArabicLetterHeh,
            ArabicCharacter::try_from('\u{0647}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_waw() {
        assert_eq!(ArabicLetterWaw.block(), "Arabic");
        assert_eq!(ArabicLetterWaw.name(), "Arabic Letter Waw");
        assert_eq!(ArabicLetterWaw.scalar_value(), '\u{0648}');

        assert_eq!(
            format!("{:?}", ArabicLetterWaw),
            "Arabic Letter Waw { \u{0648} }"
        );
        assert_eq!(format!("{}", ArabicLetterWaw), "\u{0648}");
        assert_eq!(ArabicLetterWaw, ArabicLetterWaw);
        assert_eq!(ArabicLetterWaw, '\u{0648}');
        assert_eq!(ArabicLetterWaw, "\u{0648}");
        assert_eq!(ArabicLetterWaw, "\u{0648}".to_string());
        assert_eq!(
            ArabicLetterWaw,
            ArabicCharacter::try_from('\u{0648}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_alef_maksura() {
        assert_eq!(ArabicLetterAlefMaksura.block(), "Arabic");
        assert_eq!(ArabicLetterAlefMaksura.name(), "Arabic Letter Alef Maksura");
        assert_eq!(ArabicLetterAlefMaksura.scalar_value(), '\u{0649}');

        assert_eq!(
            format!("{:?}", ArabicLetterAlefMaksura),
            "Arabic Letter Alef Maksura { \u{0649} }"
        );
        assert_eq!(format!("{}", ArabicLetterAlefMaksura), "\u{0649}");
        assert_eq!(ArabicLetterAlefMaksura, ArabicLetterAlefMaksura);
        assert_eq!(ArabicLetterAlefMaksura, '\u{0649}');
        assert_eq!(ArabicLetterAlefMaksura, "\u{0649}");
        assert_eq!(ArabicLetterAlefMaksura, "\u{0649}".to_string());
        assert_eq!(
            ArabicLetterAlefMaksura,
            ArabicCharacter::try_from('\u{0649}').unwrap()
        );
    }

    #[test]
    fn arabic_letter_yeh() {
        assert_eq!(ArabicLetterYeh.block(), "Arabic");
        assert_eq!(ArabicLetterYeh.name(), "Arabic Letter Yeh");
        assert_eq!(ArabicLetterYeh.scalar_value(), '\u{064A}');

        assert_eq!(
            format!("{:?}", ArabicLetterYeh),
            "Arabic Letter Yeh { \u{064A} }"
        );
        assert_eq!(format!("{}", ArabicLetterYeh), "\u{064A}");
        assert_eq!(ArabicLetterYeh, ArabicLetterYeh);
        assert_eq!(ArabicLetterYeh, '\u{064A}');
        assert_eq!(ArabicLetterYeh, "\u{064A}");
        assert_eq!(ArabicLetterYeh, "\u{064A}".to_string());
        assert_eq!(
            ArabicLetterYeh,
            ArabicCharacter::try_from('\u{064A}').unwrap()
        );
    }

    #[test]
    fn arabic_fathatan() {
        assert_eq!(ArabicFathatan.block(), "Arabic");
        assert_eq!(ArabicFathatan.name(), "Arabic Fathatan");
        assert_eq!(ArabicFathatan.scalar_value(), '\u{064B}');

        assert_eq!(
            format!("{:?}", ArabicFathatan),
            "Arabic Fathatan { \u{064B} }"
        );
        assert_eq!(format!("{}", ArabicFathatan), "\u{064B}");
        assert_eq!(ArabicFathatan, ArabicFathatan);
        assert_eq!(ArabicFathatan, '\u{064B}');
        assert_eq!(ArabicFathatan, "\u{064B}");
        assert_eq!(ArabicFathatan, "\u{064B}".to_string());
        assert_eq!(
            ArabicFathatan,
            ArabicCharacter::try_from('\u{064B}').unwrap()
        );
    }

    #[test]
    fn arabic_dammatan() {
        assert_eq!(ArabicDammatan.block(), "Arabic");
        assert_eq!(ArabicDammatan.name(), "Arabic Dammatan");
        assert_eq!(ArabicDammatan.scalar_value(), '\u{064C}');

        assert_eq!(
            format!("{:?}", ArabicDammatan),
            "Arabic Dammatan { \u{064C} }"
        );
        assert_eq!(format!("{}", ArabicDammatan), "\u{064C}");
        assert_eq!(ArabicDammatan, ArabicDammatan);
        assert_eq!(ArabicDammatan, '\u{064C}');
        assert_eq!(ArabicDammatan, "\u{064C}");
        assert_eq!(ArabicDammatan, "\u{064C}".to_string());
        assert_eq!(
            ArabicDammatan,
            ArabicCharacter::try_from('\u{064C}').unwrap()
        );
    }

    #[test]
    fn arabic_kasratan() {
        assert_eq!(ArabicKasratan.block(), "Arabic");
        assert_eq!(ArabicKasratan.name(), "Arabic Kasratan");
        assert_eq!(ArabicKasratan.scalar_value(), '\u{064D}');

        assert_eq!(
            format!("{:?}", ArabicKasratan),
            "Arabic Kasratan { \u{064D} }"
        );
        assert_eq!(format!("{}", ArabicKasratan), "\u{064D}");
        assert_eq!(ArabicKasratan, ArabicKasratan);
        assert_eq!(ArabicKasratan, '\u{064D}');
        assert_eq!(ArabicKasratan, "\u{064D}");
        assert_eq!(ArabicKasratan, "\u{064D}".to_string());
        assert_eq!(
            ArabicKasratan,
            ArabicCharacter::try_from('\u{064D}').unwrap()
        );
    }

    #[test]
    fn arabic_fatha() {
        assert_eq!(ArabicFatha.block(), "Arabic");
        assert_eq!(ArabicFatha.name(), "Arabic Fatha");
        assert_eq!(ArabicFatha.scalar_value(), '\u{064E}');

        assert_eq!(format!("{:?}", ArabicFatha), "Arabic Fatha { \u{064E} }");
        assert_eq!(format!("{}", ArabicFatha), "\u{064E}");
        assert_eq!(ArabicFatha, ArabicFatha);
        assert_eq!(ArabicFatha, '\u{064E}');
        assert_eq!(ArabicFatha, "\u{064E}");
        assert_eq!(ArabicFatha, "\u{064E}".to_string());
        assert_eq!(ArabicFatha, ArabicCharacter::try_from('\u{064E}').unwrap());
    }

    #[test]
    fn arabic_damma() {
        assert_eq!(ArabicDamma.block(), "Arabic");
        assert_eq!(ArabicDamma.name(), "Arabic Damma");
        assert_eq!(ArabicDamma.scalar_value(), '\u{064F}');

        assert_eq!(format!("{:?}", ArabicDamma), "Arabic Damma { \u{064F} }");
        assert_eq!(format!("{}", ArabicDamma), "\u{064F}");
        assert_eq!(ArabicDamma, ArabicDamma);
        assert_eq!(ArabicDamma, '\u{064F}');
        assert_eq!(ArabicDamma, "\u{064F}");
        assert_eq!(ArabicDamma, "\u{064F}".to_string());
        assert_eq!(ArabicDamma, ArabicCharacter::try_from('\u{064F}').unwrap());
    }

    #[test]
    fn arabic_kasra() {
        assert_eq!(ArabicKasra.block(), "Arabic");
        assert_eq!(ArabicKasra.name(), "Arabic Kasra");
        assert_eq!(ArabicKasra.scalar_value(), '\u{0650}');

        assert_eq!(format!("{:?}", ArabicKasra), "Arabic Kasra { \u{0650} }");
        assert_eq!(format!("{}", ArabicKasra), "\u{0650}");
        assert_eq!(ArabicKasra, ArabicKasra);
        assert_eq!(ArabicKasra, '\u{0650}');
        assert_eq!(ArabicKasra, "\u{0650}");
        assert_eq!(ArabicKasra, "\u{0650}".to_string());
        assert_eq!(ArabicKasra, ArabicCharacter::try_from('\u{0650}').unwrap());
    }

    #[test]
    fn arabic_shadda() {
        assert_eq!(ArabicShadda.block(), "Arabic");
        assert_eq!(ArabicShadda.name(), "Arabic Shadda");
        assert_eq!(ArabicShadda.scalar_value(), '\u{0651}');

        assert_eq!(format!("{:?}", ArabicShadda), "Arabic Shadda { \u{0651} }");
        assert_eq!(format!("{}", ArabicShadda), "\u{0651}");
        assert_eq!(ArabicShadda, ArabicShadda);
        assert_eq!(ArabicShadda, '\u{0651}');
        assert_eq!(ArabicShadda, "\u{0651}");
        assert_eq!(ArabicShadda, "\u{0651}".to_string());
        assert_eq!(ArabicShadda, ArabicCharacter::try_from('\u{0651}').unwrap());
    }

    #[test]
    fn arabic_sukun() {
        assert_eq!(ArabicSukun.block(), "Arabic");
        assert_eq!(ArabicSukun.name(), "Arabic Sukun");
        assert_eq!(ArabicSukun.scalar_value(), '\u{0652}');

        assert_eq!(format!("{:?}", ArabicSukun), "Arabic Sukun { \u{0652} }");
        assert_eq!(format!("{}", ArabicSukun), "\u{0652}");
        assert_eq!(ArabicSukun, ArabicSukun);
        assert_eq!(ArabicSukun, '\u{0652}');
        assert_eq!(ArabicSukun, "\u{0652}");
        assert_eq!(ArabicSukun, "\u{0652}".to_string());
        assert_eq!(ArabicSukun, ArabicCharacter::try_from('\u{0652}').unwrap());
    }
}
