/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
package UserDefinedErrorsPackageWithoutMatchingCatch2 {

public class BoxDimensionException extends Error {
  public function BoxDimensionException(boxErrMsg: String) {
    super(boxErrMsg);

  }

}
}
