/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import { UseMethod, useMethodBeet } from './UseMethod'
export type Uses = {
  useMethod: UseMethod
  remaining: beet.bignum
  total: beet.bignum
}

/**
 * @category userTypes
 * @category generated
 */
export const usesBeet = new beet.BeetArgsStruct<Uses>(
  [
    ['useMethod', useMethodBeet],
    ['remaining', beet.u64],
    ['total', beet.u64],
  ],
  'Uses'
)
