/*
 * Copyright 2018 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use follow::Follow;
use primitives::*;
use vtable::VTable;
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub buf: Bytes,
    pub loc: usize,
}

impl Table {
    #[inline]
    pub fn new(buf: Bytes, loc: usize) -> Self {
        Table { buf: buf, loc: loc }
    }
    #[inline]
    pub fn vtable(&self) -> VTable {
        <BackwardsSOffset<VTable>>::follow(&self.buf, self.loc)
    }
    #[inline]
    pub fn get<T: Follow>(
        &self,
        slot_byte_loc: VOffsetT,
        default: Option<T::Inner>,
    ) -> Option<T::Inner> {
        let o = self.vtable().get(slot_byte_loc) as usize;
        if o == 0 {
            return default;
        }
        Some(<T>::follow(&self.buf, self.loc + o))
    }
}

impl Follow for Table {
    type Inner = Table;
    #[inline]
    fn follow(buf: &Bytes, loc: usize) -> Self::Inner {
        Table { buf: buf.clone(), loc: loc }
    }
}

#[inline]
pub fn get_root<T: Follow>(data: &[u8]) -> T::Inner {
    <ForwardsUOffset<T>>::follow(&Bytes::from(data), 0)
}

#[inline]
pub fn get_size_prefixed_root<'a, T: Follow>(data: &[u8]) -> T::Inner {
    <SkipSizePrefix<ForwardsUOffset<T>>>::follow(&Bytes::from(data), 0)
}

#[inline]
pub fn buffer_has_identifier(data: &[u8], ident: &str, size_prefixed: bool) -> bool {
    assert_eq!(ident.len(), FILE_IDENTIFIER_LENGTH);
    let bytes = Bytes::from(data);

    let got = if size_prefixed {
        <SkipSizePrefix<SkipRootOffset<FileIdentifier>>>::follow(&bytes, 0)
    } else {
        <SkipRootOffset<FileIdentifier>>::follow(&bytes, 0)
    };

    ident.as_bytes() == got
}
