// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.
use crate::{Attribute, Type};
use leo_typed::{Circuit, Function, Identifier};

use crate::FunctionInputVariableType;
use std::fmt;

/// Stores variable definition details.
///
/// This type should be added to the variable symbol table for a resolved syntax tree.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ParameterType {
    pub identifier: Identifier,
    pub type_: Type,
    pub attributes: Vec<Attribute>,
}

impl ParameterType {
    ///
    /// Returns `true` if this variable's value can be modified.
    ///
    pub fn is_mutable(&self) -> bool {
        self.attributes.contains(&Attribute::Mutable)
    }
}

impl From<Circuit> for ParameterType {
    fn from(value: Circuit) -> Self {
        let identifier = value.circuit_name;

        ParameterType {
            identifier: identifier.clone(),
            type_: Type::Circuit(identifier),
            attributes: vec![],
        }
    }
}

impl From<Function> for ParameterType {
    fn from(value: Function) -> Self {
        let identifier = value.identifier;

        ParameterType {
            identifier: identifier.clone(),
            type_: Type::Function(identifier.clone()),
            attributes: vec![],
        }
    }
}

impl From<FunctionInputVariableType> for ParameterType {
    fn from(value: FunctionInputVariableType) -> Self {
        ParameterType {
            identifier: value.identifier,
            type_: value.type_,
            attributes: value.attributes,
        }
    }
}

impl fmt::Display for ParameterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}