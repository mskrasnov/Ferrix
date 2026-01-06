/* vmstat.rs
 *
 * Copyright 2025 Michail Krasnov <mskrasnov07@ya.ru>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! Get virtual memory statistics

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

use crate::traits::ToJson;

/// Virtual memory statistics
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct VmStat {
    pub nr_free_pages: Option<usize>,
    pub nr_zone_inactive_anon: Option<usize>,
    pub nr_zone_active_anon: Option<usize>,
    pub nr_zone_inactive_file: Option<usize>,
    pub nr_zone_active_file: Option<usize>,
    pub nr_zone_unevictable: Option<usize>,
    pub nr_zone_write_pending: Option<usize>,
    pub nr_mlock: Option<usize>,
    pub nr_bounce: Option<usize>,
    pub nr_zspages: Option<usize>,
    pub nr_free_cma: Option<usize>,
    pub numa_hit: Option<usize>,
    pub numa_miss: Option<usize>,
    pub numa_foreign: Option<usize>,
    pub numa_interleave: Option<usize>,
    pub numa_local: Option<usize>,
    pub numa_other: Option<usize>,
    pub nr_inactive_anon: Option<usize>,
    pub nr_active_anon: Option<usize>,
    pub nr_inactive_file: Option<usize>,
    pub nr_active_file: Option<usize>,
    pub nr_unevictable: Option<usize>,
    pub nr_slab_reclaimable: Option<usize>,
    pub nr_slab_unreclaimable: Option<usize>,
    pub nr_isolated_anon: Option<usize>,
    pub nr_isolated_file: Option<usize>,
    pub workingset_nodes: Option<usize>,
    pub workingset_refault_anon: Option<usize>,
    pub workingset_activate_anon: Option<usize>,
    pub workingset_activate_file: Option<usize>,
    pub workingset_restore_anon: Option<usize>,
    pub workingset_restore_file: Option<usize>,
    pub workingset_nodereclaim: Option<usize>,
    pub nr_anon_pages: Option<usize>,
    pub nr_mapped: Option<usize>,
    pub nr_file_pages: Option<usize>,
    pub nr_dirty: Option<usize>,
    pub nr_writeback: Option<usize>,
    pub nr_writeback_temp: Option<usize>,
    pub nr_shmem: Option<usize>,
    pub nr_shmem_hugepages: Option<usize>,
    pub nr_shmem_pmdmapped: Option<usize>,
    pub nr_file_hugepages: Option<usize>,
    pub nr_file_pmdmapped: Option<usize>,
    pub nr_anon_transparent_hugepages: Option<usize>,
    pub nr_vmscan_write: Option<usize>,
    pub nr_vmscan_immediate_reclaim: Option<usize>,
    pub nr_dirtied: Option<usize>,
    pub nr_written: Option<usize>,
    pub nr_throttled_written: Option<usize>,
    pub nr_kernel_misc_reclaimable: Option<usize>,
    pub nr_foll_pin_acquired: Option<usize>,
    pub nr_foll_pin_released: Option<usize>,
    pub nr_kernel_stack: Option<usize>,
    pub nr_page_table_pages: Option<usize>,
    pub nr_sec_page_table_pages: Option<usize>,
    pub nr_swapcached: Option<usize>,
    pub pgpromote_success: Option<usize>,
    pub pgpromote_candidate: Option<usize>,
    pub nr_dirty_threshold: Option<usize>,
    pub nr_dirty_background_threshold: Option<usize>,
    pub pgpgin: Option<usize>,
    pub pgpgout: Option<usize>,
    pub pswpin: Option<usize>,
    pub pswpout: Option<usize>,
    pub pgalloc_dma: Option<usize>,
    pub pgalloc_dma32: Option<usize>,
    pub pgalloc_normal: Option<usize>,
    pub pgalloc_movable: Option<usize>,
    pub pgalloc_device: Option<usize>,
    pub allocstall_dma: Option<usize>,
    pub allocstall_dma32: Option<usize>,
    pub allocstall_normal: Option<usize>,
    pub allocstall_movable: Option<usize>,
    pub allocstall_device: Option<usize>,
    pub pgskip_dma: Option<usize>,
    pub pgskip_dma32: Option<usize>,
    pub pgskip_normal: Option<usize>,
    pub pgskip_movable: Option<usize>,
    pub pgskip_device: Option<usize>,
    pub pgfree: Option<usize>,
    pub pgactivate: Option<usize>,
    pub pgdeactivate: Option<usize>,
    pub pglazyfree: Option<usize>,
    pub pgfault: Option<usize>,
    pub pgmajfault: Option<usize>,
    pub pglazyfreed: Option<usize>,
    pub pgrefill: Option<usize>,
    pub pgreuse: Option<usize>,
    pub pgsteal_kswapd: Option<usize>,
    pub pgsteal_direct: Option<usize>,
    pub pgdemote_kswapd: Option<usize>,
    pub pgdemote_direct: Option<usize>,
    pub pgscan_kswapd: Option<usize>,
    pub pgscan_direct: Option<usize>,
    pub pgscan_direct_throttle: Option<usize>,
    pub pgscan_anon: Option<usize>,
    pub pgscan_file: Option<usize>,
    pub pgsteal_anon: Option<usize>,
    pub pgsteal_file: Option<usize>,
    pub zone_reclaim_failed: Option<usize>,
    pub pginodesteal: Option<usize>,
    pub slabs_scanned: Option<usize>,
    pub kswapd_inodesteal: Option<usize>,
    pub kswapd_low_wmark_hit_quickly: Option<usize>,
    pub kswapd_high_wmark_hit_quickly: Option<usize>,
    pub pageoutrun: Option<usize>,
    pub pgrotated: Option<usize>,
    pub drop_pagecache: Option<usize>,
    pub drop_slab: Option<usize>,
    pub oom_kill: Option<usize>,
    pub numa_pte_updates: Option<usize>,
    pub numa_huge_pte_updates: Option<usize>,
    pub numa_hint_faults: Option<usize>,
    pub numa_hint_faults_local: Option<usize>,
    pub numa_pages_migrated: Option<usize>,
    pub pgmigrate_success: Option<usize>,
    pub pgmigrate_fail: Option<usize>,
    pub thp_migration_success: Option<usize>,
    pub thp_migration_fail: Option<usize>,
    pub thp_migration_split: Option<usize>,
    pub compact_migrate_scanned: Option<usize>,
    pub compact_free_scanned: Option<usize>,
    pub compact_isolated: Option<usize>,
    pub compact_stall: Option<usize>,
    pub compact_fail: Option<usize>,
    pub compact_success: Option<usize>,
    pub compact_daemon_wake: Option<usize>,
    pub compact_daemon_migrate_scanned: Option<usize>,
    pub compact_daemon_free_scanned: Option<usize>,
    pub htlb_buddy_alloc_success: Option<usize>,
    pub htlb_buddy_alloc_fail: Option<usize>,
    pub unevictable_pgs_culled: Option<usize>,
    pub unevictable_pgs_scanned: Option<usize>,
    pub unevictable_pgs_rescued: Option<usize>,
    pub unevictable_pgs_mlocked: Option<usize>,
    pub unevictable_pgs_munlocked: Option<usize>,
    pub unevictable_pgs_cleared: Option<usize>,
    pub unevictable_pgs_stranded: Option<usize>,
    pub thp_fault_alloc: Option<usize>,
    pub thp_fault_fallback: Option<usize>,
    pub thp_fault_fallback_charge: Option<usize>,
    pub thp_collapse_alloc: Option<usize>,
    pub thp_collapse_alloc_failed: Option<usize>,
    pub thp_file_alloc: Option<usize>,
    pub thp_file_fallback: Option<usize>,
    pub thp_file_mapped: Option<usize>,
    pub thp_split_page: Option<usize>,
    pub thp_split_page_failed: Option<usize>,
    pub thp_deferred_split_page: Option<usize>,
    pub thp_split_pmd: Option<usize>,
    pub thp_scan_exceed_none_pte: Option<usize>,
    pub thp_scan_exceed_swap_pte: Option<usize>,
    pub thp_scan_exceed_share_pte: Option<usize>,
    pub thp_split_pud: Option<usize>,
    pub thp_zero_page_alloc: Option<usize>,
    pub thp_zero_page_alloc_failed: Option<usize>,
    pub thp_swpout: Option<usize>,
    pub thp_swpout_fallback: Option<usize>,
    pub balloon_inflate: Option<usize>,
    pub balloon_deflate: Option<usize>,
    pub balloon_migrate: Option<usize>,
    pub swap_ra: Option<usize>,
    pub swap_ra_hit: Option<usize>,
    pub ksm_swpin_copy: Option<usize>,
    pub cow_ksm: Option<usize>,
    pub zswpin: Option<usize>,
    pub zswpout: Option<usize>,
    pub direct_map_level2_splits: Option<usize>,
    pub direct_map_level3_splits: Option<usize>,
    pub nr_unstable: Option<usize>,
}

impl VmStat {
    pub fn new() -> Result<Self> {
        let chunks = read_to_string("/proc/vmstat")?;
        let chunks = chunks
            .lines()
            .map(|item| {
                let mut items = item.trim().split_whitespace().map(|item| item.trim());
                (items.next(), items.next())
            })
            .collect::<Vec<_>>();
        let mut vmstat = VmStat::default();
        for chunk in chunks {
            match chunk {
                (Some(key), Some(val)) => {
                    let value = val.parse::<usize>()?;
                    match key {
                        "nr_free_pages" => vmstat.nr_free_pages = Some(value),
                        "nr_zone_inactive_anon" => vmstat.nr_zone_inactive_anon = Some(value),
                        "nr_zone_active_anon" => vmstat.nr_zone_active_anon = Some(value),
                        "nr_zone_inactive_file" => vmstat.nr_zone_inactive_file = Some(value),
                        "nr_zone_active_file" => vmstat.nr_zone_active_file = Some(value),
                        "nr_zone_unevictable" => vmstat.nr_zone_unevictable = Some(value),
                        "nr_zone_write_pending" => vmstat.nr_zone_write_pending = Some(value),
                        "nr_mlock" => vmstat.nr_mlock = Some(value),
                        "nr_bounce" => vmstat.nr_bounce = Some(value),
                        "nr_zspages" => vmstat.nr_zspages = Some(value),
                        "nr_free_cma" => vmstat.nr_free_cma = Some(value),
                        "numa_hit" => vmstat.numa_hit = Some(value),
                        "numa_miss" => vmstat.numa_miss = Some(value),
                        "numa_foreign" => vmstat.numa_foreign = Some(value),
                        "numa_interleave" => vmstat.numa_interleave = Some(value),
                        "numa_local" => vmstat.numa_local = Some(value),
                        "numa_other" => vmstat.numa_other = Some(value),
                        "nr_inactive_anon" => vmstat.nr_inactive_anon = Some(value),
                        "nr_active_anon" => vmstat.nr_active_anon = Some(value),
                        "nr_inactive_file" => vmstat.nr_inactive_file = Some(value),
                        "nr_active_file" => vmstat.nr_active_file = Some(value),
                        "nr_unevictable" => vmstat.nr_unevictable = Some(value),
                        "nr_slab_reclaimable" => vmstat.nr_slab_reclaimable = Some(value),
                        "nr_slab_unreclaimable" => vmstat.nr_slab_unreclaimable = Some(value),
                        "nr_isolated_anon" => vmstat.nr_isolated_anon = Some(value),
                        "nr_isolated_file" => vmstat.nr_isolated_file = Some(value),
                        "workingset_nodes" => vmstat.workingset_nodes = Some(value),
                        "workingset_refault_anon" => vmstat.workingset_refault_anon = Some(value),
                        "workingset_activate_anon" => vmstat.workingset_activate_anon = Some(value),
                        "workingset_activate_file" => vmstat.workingset_activate_file = Some(value),
                        "workingset_restore_anon" => vmstat.workingset_restore_anon = Some(value),
                        "workingset_restore_file" => vmstat.workingset_restore_file = Some(value),
                        "workingset_nodereclaim" => vmstat.workingset_nodereclaim = Some(value),
                        "nr_anon_pages" => vmstat.nr_anon_pages = Some(value),
                        "nr_mapped" => vmstat.nr_mapped = Some(value),
                        "nr_file_pages" => vmstat.nr_file_pages = Some(value),
                        "nr_dirty" => vmstat.nr_dirty = Some(value),
                        "nr_writeback" => vmstat.nr_writeback = Some(value),
                        "nr_writeback_temp" => vmstat.nr_writeback_temp = Some(value),
                        "nr_shmem" => vmstat.nr_shmem = Some(value),
                        "nr_shmem_hugepages" => vmstat.nr_shmem_hugepages = Some(value),
                        "nr_shmem_pmdmapped" => vmstat.nr_shmem_pmdmapped = Some(value),
                        "nr_file_hugepages" => vmstat.nr_file_hugepages = Some(value),
                        "nr_file_pmdmapped" => vmstat.nr_file_pmdmapped = Some(value),
                        "nr_anon_transparent_hugepages" => {
                            vmstat.nr_anon_transparent_hugepages = Some(value)
                        }
                        "nr_vmscan_write" => vmstat.nr_vmscan_write = Some(value),
                        "nr_vmscan_immediate_reclaim" => {
                            vmstat.nr_vmscan_immediate_reclaim = Some(value)
                        }
                        "nr_dirtied" => vmstat.nr_dirtied = Some(value),
                        "nr_written" => vmstat.nr_written = Some(value),
                        "nr_throttled_written" => vmstat.nr_throttled_written = Some(value),
                        "nr_kernel_misc_reclaimable" => {
                            vmstat.nr_kernel_misc_reclaimable = Some(value)
                        }
                        "nr_foll_pin_acquired" => vmstat.nr_foll_pin_acquired = Some(value),
                        "nr_foll_pin_released" => vmstat.nr_foll_pin_released = Some(value),
                        "nr_kernel_stack" => vmstat.nr_kernel_stack = Some(value),
                        "nr_page_table_pages" => vmstat.nr_page_table_pages = Some(value),
                        "nr_sec_page_table_pages" => vmstat.nr_sec_page_table_pages = Some(value),
                        "nr_swapcached" => vmstat.nr_swapcached = Some(value),
                        "pgpromote_success" => vmstat.pgpromote_success = Some(value),
                        "pgpromote_candidate" => vmstat.pgpromote_candidate = Some(value),
                        "nr_dirty_threshold" => vmstat.nr_dirty_threshold = Some(value),
                        "nr_dirty_background_threshold" => {
                            vmstat.nr_dirty_background_threshold = Some(value)
                        }
                        "pgpgin" => vmstat.pgpgin = Some(value),
                        "pgpgout" => vmstat.pgpgout = Some(value),
                        "pswpin" => vmstat.pswpin = Some(value),
                        "pswpout" => vmstat.pswpout = Some(value),
                        "pgalloc_dma" => vmstat.pgalloc_dma = Some(value),
                        "pgalloc_dma32" => vmstat.pgalloc_dma32 = Some(value),
                        "pgalloc_normal" => vmstat.pgalloc_normal = Some(value),
                        "pgalloc_movable" => vmstat.pgalloc_movable = Some(value),
                        "pgalloc_device" => vmstat.pgalloc_device = Some(value),
                        "allocstall_dma" => vmstat.allocstall_dma = Some(value),
                        "allocstall_dma32" => vmstat.allocstall_dma32 = Some(value),
                        "allocstall_normal" => vmstat.allocstall_normal = Some(value),
                        "allocstall_movable" => vmstat.allocstall_movable = Some(value),
                        "allocstall_device" => vmstat.allocstall_device = Some(value),
                        "pgskip_dma" => vmstat.pgskip_dma = Some(value),
                        "pgskip_dma32" => vmstat.pgskip_dma32 = Some(value),
                        "pgskip_normal" => vmstat.pgskip_normal = Some(value),
                        "pgskip_movable" => vmstat.pgskip_movable = Some(value),
                        "pgskip_device" => vmstat.pgskip_device = Some(value),
                        "pgfree" => vmstat.pgfree = Some(value),
                        "pgactivate" => vmstat.pgactivate = Some(value),
                        "pgdeactivate" => vmstat.pgdeactivate = Some(value),
                        "pglazyfree" => vmstat.pglazyfree = Some(value),
                        "pgfault" => vmstat.pgfault = Some(value),
                        "pgmajfault" => vmstat.pgmajfault = Some(value),
                        "pglazyfreed" => vmstat.pglazyfreed = Some(value),
                        "pgrefill" => vmstat.pgrefill = Some(value),
                        "pgreuse" => vmstat.pgreuse = Some(value),
                        "pgsteal_kswapd" => vmstat.pgsteal_kswapd = Some(value),
                        "pgsteal_direct" => vmstat.pgsteal_direct = Some(value),
                        "pgdemote_kswapd" => vmstat.pgdemote_kswapd = Some(value),
                        "pgdemote_direct" => vmstat.pgdemote_direct = Some(value),
                        "pgscan_kswapd" => vmstat.pgscan_kswapd = Some(value),
                        "pgscan_direct" => vmstat.pgscan_direct = Some(value),
                        "pgscan_direct_throttle" => vmstat.pgscan_direct_throttle = Some(value),
                        "pgscan_anon" => vmstat.pgscan_anon = Some(value),
                        "pgscan_file" => vmstat.pgscan_file = Some(value),
                        "pgsteal_anon" => vmstat.pgsteal_anon = Some(value),
                        "pgsteal_file" => vmstat.pgsteal_file = Some(value),
                        "zone_reclaim_failed" => vmstat.zone_reclaim_failed = Some(value),
                        "pginodesteal" => vmstat.pginodesteal = Some(value),
                        "slabs_scanned" => vmstat.slabs_scanned = Some(value),
                        "kswapd_inodesteal" => vmstat.kswapd_inodesteal = Some(value),
                        "kswapd_low_wmark_hit_quickly" => {
                            vmstat.kswapd_low_wmark_hit_quickly = Some(value)
                        }
                        "kswapd_high_wmark_hit_quickly" => {
                            vmstat.kswapd_high_wmark_hit_quickly = Some(value)
                        }
                        "pageoutrun" => vmstat.pageoutrun = Some(value),
                        "pgrotated" => vmstat.pgrotated = Some(value),
                        "drop_pagecache" => vmstat.drop_pagecache = Some(value),
                        "drop_slab" => vmstat.drop_slab = Some(value),
                        "oom_kill" => vmstat.oom_kill = Some(value),
                        "numa_pte_updates" => vmstat.numa_pte_updates = Some(value),
                        "numa_huge_pte_updates" => vmstat.numa_huge_pte_updates = Some(value),
                        "numa_hint_faults" => vmstat.numa_hint_faults = Some(value),
                        "numa_hint_faults_local" => vmstat.numa_hint_faults_local = Some(value),
                        "numa_pages_migrated" => vmstat.numa_pages_migrated = Some(value),
                        "pgmigrate_success" => vmstat.pgmigrate_success = Some(value),
                        "pgmigrate_fail" => vmstat.pgmigrate_fail = Some(value),
                        "thp_migration_success" => vmstat.thp_migration_success = Some(value),
                        "thp_migration_fail" => vmstat.thp_migration_fail = Some(value),
                        "thp_migration_split" => vmstat.thp_migration_split = Some(value),
                        "compact_migrate_scanned" => vmstat.compact_migrate_scanned = Some(value),
                        "compact_free_scanned" => vmstat.compact_free_scanned = Some(value),
                        "compact_isolated" => vmstat.compact_isolated = Some(value),
                        "compact_stall" => vmstat.compact_stall = Some(value),
                        "compact_fail" => vmstat.compact_fail = Some(value),
                        "compact_success" => vmstat.compact_success = Some(value),
                        "compact_daemon_wake" => vmstat.compact_daemon_wake = Some(value),
                        "compact_daemon_migrate_scanned" => {
                            vmstat.compact_daemon_migrate_scanned = Some(value)
                        }
                        "compact_daemon_free_scanned" => {
                            vmstat.compact_daemon_free_scanned = Some(value)
                        }
                        "htlb_buddy_alloc_success" => vmstat.htlb_buddy_alloc_success = Some(value),
                        "htlb_buddy_alloc_fail" => vmstat.htlb_buddy_alloc_fail = Some(value),
                        "unevictable_pgs_culled" => vmstat.unevictable_pgs_culled = Some(value),
                        "unevictable_pgs_scanned" => vmstat.unevictable_pgs_scanned = Some(value),
                        "unevictable_pgs_rescued" => vmstat.unevictable_pgs_rescued = Some(value),
                        "unevictable_pgs_mlocked" => vmstat.unevictable_pgs_mlocked = Some(value),
                        "unevictable_pgs_munlocked" => {
                            vmstat.unevictable_pgs_munlocked = Some(value)
                        }
                        "unevictable_pgs_cleared" => vmstat.unevictable_pgs_cleared = Some(value),
                        "unevictable_pgs_stranded" => vmstat.unevictable_pgs_stranded = Some(value),
                        "thp_fault_alloc" => vmstat.thp_fault_alloc = Some(value),
                        "thp_fault_fallback" => vmstat.thp_fault_fallback = Some(value),
                        "thp_fault_fallback_charge" => {
                            vmstat.thp_fault_fallback_charge = Some(value)
                        }
                        "thp_collapse_alloc" => vmstat.thp_collapse_alloc = Some(value),
                        "thp_collapse_alloc_failed" => {
                            vmstat.thp_collapse_alloc_failed = Some(value)
                        }
                        "thp_file_alloc" => vmstat.thp_file_alloc = Some(value),
                        "thp_file_fallback" => vmstat.thp_file_fallback = Some(value),
                        "thp_file_mapped" => vmstat.thp_file_mapped = Some(value),
                        "thp_split_page" => vmstat.thp_split_page = Some(value),
                        "thp_split_page_failed" => vmstat.thp_split_page_failed = Some(value),
                        "thp_deferred_split_page" => vmstat.thp_deferred_split_page = Some(value),
                        "thp_split_pmd" => vmstat.thp_split_pmd = Some(value),
                        "thp_scan_exceed_none_pte" => vmstat.thp_scan_exceed_none_pte = Some(value),
                        "thp_scan_exceed_swap_pte" => vmstat.thp_scan_exceed_swap_pte = Some(value),
                        "thp_scan_exceed_share_pte" => {
                            vmstat.thp_scan_exceed_share_pte = Some(value)
                        }
                        "thp_split_pud" => vmstat.thp_split_pud = Some(value),
                        "thp_zero_page_alloc" => vmstat.thp_zero_page_alloc = Some(value),
                        "thp_zero_page_alloc_failed" => {
                            vmstat.thp_zero_page_alloc_failed = Some(value)
                        }
                        "thp_swpout" => vmstat.thp_swpout = Some(value),
                        "thp_swpout_fallback" => vmstat.thp_swpout_fallback = Some(value),
                        "balloon_inflate" => vmstat.balloon_inflate = Some(value),
                        "balloon_deflate" => vmstat.balloon_deflate = Some(value),
                        "balloon_migrate" => vmstat.balloon_migrate = Some(value),
                        "swap_ra" => vmstat.swap_ra = Some(value),
                        "swap_ra_hit" => vmstat.swap_ra_hit = Some(value),
                        "ksm_swpin_copy" => vmstat.ksm_swpin_copy = Some(value),
                        "cow_ksm" => vmstat.cow_ksm = Some(value),
                        "zswpin" => vmstat.zswpin = Some(value),
                        "zswpout" => vmstat.zswpout = Some(value),
                        "direct_map_level2_splits" => vmstat.direct_map_level2_splits = Some(value),
                        "direct_map_level3_splits" => vmstat.direct_map_level3_splits = Some(value),
                        "nr_unstable" => vmstat.nr_unstable = Some(value),
                        _ => continue,
                    }
                }
                _ => {}
            }
        }
        Ok(vmstat)
    }
}

impl ToJson for VmStat {}
