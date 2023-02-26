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

//! # The Arabic Script Library
//!
//! **The Arabic Script Library** provides a clear, legible, and universally usable representation
//! of all the standard characters of the Arabic script.
//!
//! Each letter, diacritical mark, numerical digit, and additional symbol is available under its
//! standardized Unicode name, providing easy access to all Arabic characters, with none of the
//! hassle of manipulating primitive Rust characters or Unicode code points.

pub use crate::core::ArabicCharacter;
pub use crate::core::ArabicCharacter::ArabicDamma;
pub use crate::core::ArabicCharacter::ArabicDammatan;
pub use crate::core::ArabicCharacter::ArabicFatha;
pub use crate::core::ArabicCharacter::ArabicFathatan;
pub use crate::core::ArabicCharacter::ArabicKasra;
pub use crate::core::ArabicCharacter::ArabicKasratan;
pub use crate::core::ArabicCharacter::ArabicLetterAin;
pub use crate::core::ArabicCharacter::ArabicLetterAlef;
pub use crate::core::ArabicCharacter::ArabicLetterAlefMaksura;
pub use crate::core::ArabicCharacter::ArabicLetterAlefWithHamzaAbove;
pub use crate::core::ArabicCharacter::ArabicLetterAlefWithHamzaBelow;
pub use crate::core::ArabicCharacter::ArabicLetterAlefWithMaddaAbove;
pub use crate::core::ArabicCharacter::ArabicLetterBeh;
pub use crate::core::ArabicCharacter::ArabicLetterDad;
pub use crate::core::ArabicCharacter::ArabicLetterDal;
pub use crate::core::ArabicCharacter::ArabicLetterFeh;
pub use crate::core::ArabicCharacter::ArabicLetterGhain;
pub use crate::core::ArabicCharacter::ArabicLetterHah;
pub use crate::core::ArabicCharacter::ArabicLetterHamza;
pub use crate::core::ArabicCharacter::ArabicLetterHeh;
pub use crate::core::ArabicCharacter::ArabicLetterJeem;
pub use crate::core::ArabicCharacter::ArabicLetterKaf;
pub use crate::core::ArabicCharacter::ArabicLetterKhah;
pub use crate::core::ArabicCharacter::ArabicLetterLam;
pub use crate::core::ArabicCharacter::ArabicLetterMeem;
pub use crate::core::ArabicCharacter::ArabicLetterNoon;
pub use crate::core::ArabicCharacter::ArabicLetterQaf;
pub use crate::core::ArabicCharacter::ArabicLetterReh;
pub use crate::core::ArabicCharacter::ArabicLetterSad;
pub use crate::core::ArabicCharacter::ArabicLetterSeen;
pub use crate::core::ArabicCharacter::ArabicLetterSheen;
pub use crate::core::ArabicCharacter::ArabicLetterTah;
pub use crate::core::ArabicCharacter::ArabicLetterTeh;
pub use crate::core::ArabicCharacter::ArabicLetterTehMarbuta;
pub use crate::core::ArabicCharacter::ArabicLetterThal;
pub use crate::core::ArabicCharacter::ArabicLetterTheh;
pub use crate::core::ArabicCharacter::ArabicLetterWaw;
pub use crate::core::ArabicCharacter::ArabicLetterWawWithHamzaAbove;
pub use crate::core::ArabicCharacter::ArabicLetterYeh;
pub use crate::core::ArabicCharacter::ArabicLetterYehWithHamzaAbove;
pub use crate::core::ArabicCharacter::ArabicLetterZah;
pub use crate::core::ArabicCharacter::ArabicLetterZain;
pub use crate::core::ArabicCharacter::ArabicShadda;
pub use crate::core::ArabicCharacter::ArabicSukun;
pub use crate::core::ArabicCharacter::ArabicTatweel;
pub use crate::core::UnicodeCharacter;

mod core;
