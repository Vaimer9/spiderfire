/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

declare function readBinary(path: string): Uint8Array;

declare function readString(path: string): string;

declare function readDir(path: string): string[];

declare function write(path: string, contents: string): boolean;

declare function createDir(path: string): boolean;

declare function createDirRecursive(path: string): boolean;

declare function removeFile(path: string): boolean;

declare function removeDir(path: string): boolean;

declare function removeDirRecursive(path: string): boolean;

declare function copy(from: string, to: string): boolean;

declare function rename(from: string, to: string): boolean;

declare function softLink(original: string, link: string): boolean;

declare function hardLink(original: string, link: string): boolean;
