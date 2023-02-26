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

use crate::core::UnicodeCharacter;

/// Represents the Arabic Letter Alef With Hamza Below.
pub struct ArabicLetterAlefWithHamzaBelow;

impl ArabicLetterAlefWithHamzaBelow {
    /// Returns a new instance of the Arabic Letter Alef With Hamza Below.
    pub fn new() -> Self {
        Self
    }
}

impl UnicodeCharacter for ArabicLetterAlefWithHamzaBelow {
    fn block(&self) -> &'static str {
        "Arabic"
    }

    fn name(&self) -> &'static str {
        "Arabic Letter Alef With Hamza Below"
    }

    fn scalar_value(&self) -> char {
        '\u{0625}'
    }
}
