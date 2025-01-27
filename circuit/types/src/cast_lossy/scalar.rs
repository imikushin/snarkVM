// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<E: Environment> CastLossy<Address<E>> for Scalar<E> {
    /// Casts a `Scalar` to an `Address`.
    #[inline]
    fn cast_lossy(&self) -> Address<E> {
        self.cast()
    }
}

impl<E: Environment> CastLossy<Boolean<E>> for Scalar<E> {
    /// Casts a `Scalar` to a `Boolean`.
    #[inline]
    fn cast_lossy(&self) -> Boolean<E> {
        match self.to_bits_be().pop() {
            Some(bit) => bit,
            None => E::halt("Failed to retrieve the LSB from the field element."),
        }
    }
}

impl<E: Environment> CastLossy<Group<E>> for Scalar<E> {
    /// Casts a `Scalar` to a `Group`.
    #[inline]
    fn cast_lossy(&self) -> Group<E> {
        self.cast()
    }
}

impl<E: Environment> CastLossy<Field<E>> for Scalar<E> {
    /// Casts a `Scalar` to a `Field`.
    #[inline]
    fn cast_lossy(&self) -> Field<E> {
        self.cast()
    }
}

impl<E: Environment, I: IntegerType> CastLossy<Integer<E, I>> for Scalar<E> {
    /// Casts a `Scalar` to an `Integer`.
    #[inline]
    fn cast_lossy(&self) -> Integer<E, I> {
        let bits_le = self.to_bits_le();
        // Use the appropriate lower bits.
        Integer::<E, I>::from_bits_le(&bits_le[0..(I::BITS as usize)])
    }
}

impl<E: Environment> CastLossy<Scalar<E>> for Scalar<E> {
    /// Casts a `Scalar` to a `Scalar`.
    #[inline]
    fn cast_lossy(&self) -> Scalar<E> {
        self.cast()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_helpers::impl_check_cast, CastLossy as CircuitCast};

    use console::{
        network::Testnet3,
        prelude::{One, TestRng, Uniform, Zero},
        types::CastLossy as ConsoleCast,
    };
    use snarkvm_circuit_environment::{count_is, count_less_than, Circuit, Eject, Inject, Mode, UpdatableCount};

    use std::fmt::Debug;

    const ITERATIONS: usize = 100;

    fn sample_values(i: usize, mode: Mode, rng: &mut TestRng) -> (console::types::Scalar<Testnet3>, Scalar<Circuit>) {
        let console_value = match i {
            0 => console::types::Scalar::<Testnet3>::zero(),
            1 => console::types::Scalar::<Testnet3>::one(),
            _ => Uniform::rand(rng),
        };
        let circuit_value = Scalar::<Circuit>::new(mode, console_value);
        (console_value, circuit_value)
    }

    impl_check_cast!(cast_lossy, Scalar<Circuit>, console::types::Scalar::<Testnet3>);

    #[test]
    fn test_scalar_to_address() {
        check_cast::<Address<Circuit>, console::types::Address<Testnet3>>(
            Mode::Constant,
            count_less_than!(11, 0, 0, 0),
        );
        check_cast::<Address<Circuit>, console::types::Address<Testnet3>>(Mode::Public, count_is!(4, 0, 15, 13));
        check_cast::<Address<Circuit>, console::types::Address<Testnet3>>(Mode::Private, count_is!(4, 0, 15, 13));
    }

    #[test]
    fn test_scalar_to_boolean() {
        check_cast::<Boolean<Circuit>, console::types::Boolean<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<Boolean<Circuit>, console::types::Boolean<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<Boolean<Circuit>, console::types::Boolean<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }

    #[test]
    fn test_scalar_to_field() {
        check_cast::<Field<Circuit>, console::types::Field<Testnet3>>(Mode::Constant, count_is!(0, 0, 0, 0));
        check_cast::<Field<Circuit>, console::types::Field<Testnet3>>(Mode::Public, count_is!(0, 0, 0, 0));
        check_cast::<Field<Circuit>, console::types::Field<Testnet3>>(Mode::Private, count_is!(0, 0, 0, 0));
    }

    #[test]
    fn test_scalar_to_group() {
        check_cast::<Group<Circuit>, console::types::Group<Testnet3>>(Mode::Constant, count_less_than!(11, 0, 0, 0));
        check_cast::<Group<Circuit>, console::types::Group<Testnet3>>(Mode::Public, count_is!(4, 0, 15, 13));
        check_cast::<Group<Circuit>, console::types::Group<Testnet3>>(Mode::Private, count_is!(4, 0, 15, 13));
    }

    #[test]
    fn test_scalar_to_i8() {
        check_cast::<I8<Circuit>, console::types::I8<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<I8<Circuit>, console::types::I8<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<I8<Circuit>, console::types::I8<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }

    #[test]
    fn test_scalar_to_i16() {
        check_cast::<I16<Circuit>, console::types::I16<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<I16<Circuit>, console::types::I16<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<I16<Circuit>, console::types::I16<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }

    #[test]
    fn test_scalar_to_i32() {
        check_cast::<I32<Circuit>, console::types::I32<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<I32<Circuit>, console::types::I32<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<I32<Circuit>, console::types::I32<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }

    #[test]
    fn test_scalar_to_i64() {
        check_cast::<I64<Circuit>, console::types::I64<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<I64<Circuit>, console::types::I64<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<I64<Circuit>, console::types::I64<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }

    #[test]
    fn test_scalar_to_i128() {
        check_cast::<I128<Circuit>, console::types::I128<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<I128<Circuit>, console::types::I128<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<I128<Circuit>, console::types::I128<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }

    #[test]
    fn test_scalar_to_scalar() {
        check_cast::<Scalar<Circuit>, console::types::Scalar<Testnet3>>(Mode::Constant, count_is!(0, 0, 0, 0));
        check_cast::<Scalar<Circuit>, console::types::Scalar<Testnet3>>(Mode::Public, count_is!(0, 0, 0, 0));
        check_cast::<Scalar<Circuit>, console::types::Scalar<Testnet3>>(Mode::Private, count_is!(0, 0, 0, 0));
    }

    #[test]
    fn test_scalar_to_u8() {
        check_cast::<U8<Circuit>, console::types::U8<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<U8<Circuit>, console::types::U8<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<U8<Circuit>, console::types::U8<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }

    #[test]
    fn test_scalar_to_u16() {
        check_cast::<U16<Circuit>, console::types::U16<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<U16<Circuit>, console::types::U16<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<U16<Circuit>, console::types::U16<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }

    #[test]
    fn test_scalar_to_u32() {
        check_cast::<U32<Circuit>, console::types::U32<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<U32<Circuit>, console::types::U32<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<U32<Circuit>, console::types::U32<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }

    #[test]
    fn test_scalar_to_u64() {
        check_cast::<U64<Circuit>, console::types::U64<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<U64<Circuit>, console::types::U64<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<U64<Circuit>, console::types::U64<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }

    #[test]
    fn test_scalar_to_u128() {
        check_cast::<U128<Circuit>, console::types::U128<Testnet3>>(Mode::Constant, count_is!(251, 0, 0, 0));
        check_cast::<U128<Circuit>, console::types::U128<Testnet3>>(Mode::Public, count_is!(0, 0, 501, 503));
        check_cast::<U128<Circuit>, console::types::U128<Testnet3>>(Mode::Private, count_is!(0, 0, 501, 503));
    }
}
