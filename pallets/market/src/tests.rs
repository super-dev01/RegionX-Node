// This file is part of RegionX.
//
// RegionX is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RegionX is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RegionX.  If not, see <https://www.gnu.org/licenses/>.

use crate::{mock::*, *};
use pallet_broker::CoreMask;

#[test]
fn calculate_region_price_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(
			Market::calculate_region_price(
				RegionId { begin: 2, core: 0, mask: CoreMask::complete() },
				RegionRecordOf::<u64, u64> { end: 10, owner: 1, paid: None },
				10 // timeslice price
			),
			80 // 8 * 10
		);
	});
}
