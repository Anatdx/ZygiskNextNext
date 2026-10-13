/*
 * This file is part of Zygisk Next Next.
 *
 * Zygisk Next Next is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Zygisk Next Next is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Zygisk Next Next. If not, see <https://www.gnu.org/licenses/>.
 *
 * Copyright (C) 2026 VeryBaaad <verybaaad@outlook.com>
 */

use std::fs;
use std::path::Path;

use machikado_rs::{load_folder_files, verify};

use crate::daemon::MODULE_ID;

const ORG_PUBLIC_KEY: [u8; 32] = [
    123, 192, 97, 178, 44, 86, 95, 77, 138, 95, 46, 239, 113, 227, 151, 200, 0, 120, 148, 124, 95,
    104, 121, 88, 173, 152, 219, 116, 71, 200, 239, 190,
];

pub(crate) fn verify_module(module_dir: &Path) -> Result<(), String> {
    let entries = load_folder_files(
        module_dir,
        &[],
        &["machikado", "module.prop", "remove", "disable", "update"],
        None,
    )
    .map_err(|error| error.to_string())?;

    let machikado = fs::read(module_dir.join("machikado")).map_err(|error| error.to_string())?;
    let mazoku = fs::read(module_dir.join("mazoku")).map_err(|error| error.to_string())?;

    let (ok, error) = verify(&machikado, &mazoku, &entries, MODULE_ID, &ORG_PUBLIC_KEY);
    if ok {
        Ok(())
    } else {
        Err(error.map_or_else(
            || "signature verification failed".to_owned(),
            |error| error.to_string(),
        ))
    }
}

pub(crate) fn restore_prop(module_dir: &Path) {
    let original = module_dir.join("module.prop.orig");
    if original.is_file() {
        let _ = fs::copy(&original, module_dir.join("module.prop"));
    }
}
