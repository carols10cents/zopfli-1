use libc::{c_void, c_uint, c_double, c_int, size_t};

use util::{ZopfliGetLengthSymbol, ZopfliGetLengthExtraBits, ZopfliGetDistExtraBits, ZOPFLI_NUM_LL, ZOPFLI_NUM_D};

#[repr(C)]
#[derive(Copy)]
pub struct SymbolStats {
  /* The literal and length symbols. */
  litlens: [size_t; ZOPFLI_NUM_LL],
  /* The 32 unique dist symbols, not the 32768 possible dists. */
  dists: [size_t; ZOPFLI_NUM_D],

  /* Length of each lit/len symbol in bits. */
  ll_symbols: [c_double; ZOPFLI_NUM_LL],
  /* Length of each dist symbol in bits. */
  d_symbols: [c_double; ZOPFLI_NUM_D],
}

impl SymbolStats {
    pub fn new() -> SymbolStats {
        SymbolStats {
            litlens: [0; ZOPFLI_NUM_LL],
            dists: [0; ZOPFLI_NUM_D],
            ll_symbols: [0.0; ZOPFLI_NUM_LL],
            d_symbols: [0.0; ZOPFLI_NUM_D],
        }
    }
}

impl Clone for SymbolStats {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub extern fn symbol_stats_new() -> *mut SymbolStats {
    Box::into_raw(Box::new(SymbolStats::new()))
}

#[no_mangle]
pub extern fn copy_stats(source_ptr: *mut SymbolStats, dest_ptr: *mut SymbolStats) {
    let source = unsafe {
        assert!(!source_ptr.is_null());
        &mut *source_ptr
    };
    let dest = unsafe {
        assert!(!dest_ptr.is_null());
        &mut *dest_ptr
    };
    *dest = *source;
}

/// Adds the bit lengths.
#[no_mangle]
pub extern fn add_weighed_stat_freqs(stats1_ptr: *mut SymbolStats, w1: c_double, stats2_ptr: *mut SymbolStats, w2: c_double, result_ptr: *mut SymbolStats) {
    let stats1 = unsafe {
        assert!(!stats1_ptr.is_null());
        &mut *stats1_ptr
    };
    let stats2 = unsafe {
        assert!(!stats2_ptr.is_null());
        &mut *stats2_ptr
    };
    let result = unsafe {
        assert!(!result_ptr.is_null());
        &mut *result_ptr
    };

    for i in 0..ZOPFLI_NUM_LL {
        result.litlens[i] = (stats1.litlens[i] as c_double * w1 + stats2.litlens[i] as c_double * w2) as size_t;
    }
    for i in 0..ZOPFLI_NUM_D {
        result.dists[i] = (stats1.dists[i] as c_double * w1 + stats2.dists[i] as c_double * w2) as size_t;
    }
    result.litlens[256] = 1; // End symbol.
}

#[no_mangle]
#[allow(non_snake_case)]
/// Cost model which should exactly match fixed tree.
/// type: CostModelFun
pub extern fn GetCostFixed(litlen: c_uint, dist: c_uint, _unused: c_void) -> c_double {
    let result = if dist == 0 {
        if litlen <= 143 {
            8
        } else {
            9
        }
    } else {
        let dbits = ZopfliGetDistExtraBits(dist as c_int);
        let lbits = ZopfliGetLengthExtraBits(litlen as c_int);
        let lsym = ZopfliGetLengthSymbol(litlen as c_int);
        let mut cost = 0;
        if lsym <= 279 {
            cost += 7;
        } else {
            cost += 8;
        }
        cost += 5;  // Every dist symbol has length 5.
        cost + dbits + lbits
    };
    result as c_double
}
