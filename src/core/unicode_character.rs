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

/// This trait ensures that a character exposes its Unicode owning block, name, and scalar value.
pub trait UnicodeCharacter {
    /// Returns the Unicode block of the character.
    fn block(&self) -> &'static str;

    /// Returns the Unicode name of the character.
    fn name(&self) -> &'static str;

    /// Returns the Unicode scalar value of the character, which is equivalent to a Rust [`char`].
    fn scalar_value(&self) -> char;
}
