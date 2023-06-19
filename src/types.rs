use core::ops::{Add, Div, Mul, Rem, Sub};
use nanos_sdk::bindings::{
    cx_bn_add, cx_bn_alloc, cx_bn_alloc_init, cx_bn_destroy, cx_bn_export, cx_bn_lock,
    cx_bn_mod_add, cx_bn_mod_invert_nprime, cx_bn_mod_mul, cx_bn_mod_sub, cx_bn_reduce, cx_bn_t,
    cx_bn_unlock,
};
use nanos_sdk::string::String;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FieldElement {
    pub value: [u8; 32],
}

impl FieldElement {
    pub const INVOKE: FieldElement = FieldElement {
        value: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0x6e,
            0x76, 0x6f, 0x6b, 0x65,
        ],
    };

    pub const ZERO: FieldElement = FieldElement { value: [0u8; 32] };

    pub const ONE: FieldElement = FieldElement {
        value: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ],
    };

    pub const TWO: FieldElement = FieldElement {
        value: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02,
        ],
    };

    pub fn new() -> Self {
        Self { value: [0u8; 32] }
    }

    pub fn clear(&mut self) {
        self.value.fill(0);
    }

    pub fn copy_from(&mut self, f: &FieldElement) {
        self.value.copy_from_slice(&f.value);
    }

    pub fn inverse(&self) -> FieldElement {
        let mut result_bn: cx_bn_t = Default::default();
        let mut self_bn: cx_bn_t = Default::default();
        let mut p_bn: cx_bn_t = Default::default();
        let mut result = FieldElement::new();

        unsafe {
            cx_bn_lock(32, 0);

            // Initialize and set self_bn
            cx_bn_alloc_init(&mut self_bn, 32, self.value[..].as_ptr(), 32);
            cx_bn_alloc_init(&mut p_bn, 32, P.value[..].as_ptr(), 32);

            // Allocate space for result_bn
            cx_bn_alloc(&mut result_bn, 32);

            // Perform inversion
            cx_bn_mod_invert_nprime(result_bn, self_bn, p_bn);

            // Export result_bn to result FieldElement
            cx_bn_export(result_bn, result.value[..].as_mut_ptr(), 32);

            // Destroy used bignums
            cx_bn_destroy(&mut self_bn);
            cx_bn_destroy(&mut result_bn);
            cx_bn_destroy(&mut p_bn);

            cx_bn_unlock();
        }

        result
    }
}

const P: FieldElement = FieldElement {
    value: [
        08, 00, 00, 00, 00, 00, 00, 11, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        00, 00, 00, 00, 00, 00, 00, 00, 01,
    ],
};

impl<'a, 'b> Add<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn add(self, other: &'b FieldElement) -> FieldElement {
        let mut result_bn: cx_bn_t = Default::default();
        let mut self_bn: cx_bn_t = Default::default();
        let mut other_bn: cx_bn_t = Default::default();
        let mut p_bn: cx_bn_t = Default::default();
        let mut result = FieldElement::new();

        unsafe {
            cx_bn_lock(32, 0);

            // Initialize and set self_bn and other_bn
            cx_bn_alloc_init(&mut self_bn, 32, self.value[..].as_ptr(), 32);
            cx_bn_alloc_init(&mut other_bn, 32, other.value[..].as_ptr(), 32);
            cx_bn_alloc_init(&mut p_bn, 32, P.value[..].as_ptr(), 32);

            // Allocate space for result_bn
            cx_bn_alloc(&mut result_bn, 32);

            // Perform addition
            cx_bn_mod_add(result_bn, self_bn, other_bn, p_bn);

            // Export result_bn to result FieldElement
            cx_bn_export(result_bn, result.value[..].as_mut_ptr(), 32);

            // Destroy used bignums
            cx_bn_destroy(&mut self_bn);
            cx_bn_destroy(&mut other_bn);
            cx_bn_destroy(&mut result_bn);
            cx_bn_destroy(&mut p_bn);

            cx_bn_unlock();
        }

        result
    }
}

impl<'a, 'b> Mul<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn mul(self, other: &'b FieldElement) -> FieldElement {
        let mut result_bn: cx_bn_t = Default::default();
        let mut self_bn: cx_bn_t = Default::default();
        let mut other_bn: cx_bn_t = Default::default();
        let mut p_bn: cx_bn_t = Default::default();
        let mut result = FieldElement::new();

        unsafe {
            cx_bn_lock(32, 0);

            cx_bn_alloc_init(&mut self_bn, 32, self.value[..].as_ptr(), 32);
            cx_bn_alloc_init(&mut other_bn, 32, other.value[..].as_ptr(), 32);
            cx_bn_alloc_init(&mut p_bn, 32, P.value[..].as_ptr(), 32);

            cx_bn_alloc(&mut result_bn, 32);

            cx_bn_mod_mul(result_bn, self_bn, other_bn, p_bn);

            cx_bn_export(result_bn, result.value[..].as_mut_ptr(), 32);

            cx_bn_destroy(&mut self_bn);
            cx_bn_destroy(&mut other_bn);
            cx_bn_destroy(&mut result_bn);
            cx_bn_destroy(&mut p_bn);

            cx_bn_unlock();
        }

        result
    }
}

impl<'a, 'b> Sub<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn sub(self, other: &'b FieldElement) -> FieldElement {
        let mut result_bn: cx_bn_t = Default::default();
        let mut self_bn: cx_bn_t = Default::default();
        let mut other_bn: cx_bn_t = Default::default();
        let mut p_bn: cx_bn_t = Default::default();
        let mut result = FieldElement::new();

        unsafe {
            cx_bn_lock(32, 0);

            cx_bn_alloc_init(&mut self_bn, 32, self.value[..].as_ptr(), 32);
            cx_bn_alloc_init(&mut other_bn, 32, other.value[..].as_ptr(), 32);
            cx_bn_alloc_init(&mut p_bn, 32, P.value[..].as_ptr(), 32);

            cx_bn_alloc(&mut result_bn, 32);

            cx_bn_mod_sub(result_bn, self_bn, other_bn, p_bn);

            cx_bn_export(result_bn, result.value[..].as_mut_ptr(), 32);

            cx_bn_destroy(&mut self_bn);
            cx_bn_destroy(&mut other_bn);
            cx_bn_destroy(&mut result_bn);
            cx_bn_destroy(&mut p_bn);

            cx_bn_unlock();
        }

        result
    }
}

impl<'a, 'b> Rem<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn rem(self, other: &'b FieldElement) -> FieldElement {
        let mut result_bn: cx_bn_t = Default::default();
        let mut self_bn: cx_bn_t = Default::default();
        let mut other_bn: cx_bn_t = Default::default();
        let mut result = FieldElement::new();

        unsafe {
            cx_bn_lock(32, 0);

            cx_bn_alloc_init(&mut self_bn, 32, self.value[..].as_ptr(), 32);
            cx_bn_alloc_init(&mut other_bn, 32, other.value[..].as_ptr(), 32);

            cx_bn_alloc(&mut result_bn, 32);

            cx_bn_reduce(result_bn, self_bn, other_bn);

            cx_bn_export(result_bn, result.value[..].as_mut_ptr(), 32);

            cx_bn_destroy(&mut self_bn);
            cx_bn_destroy(&mut other_bn);
            cx_bn_destroy(&mut result_bn);

            cx_bn_unlock();
        }

        result
    }
}

impl<'a, 'b> Div<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn div(self, other: &'b FieldElement) -> FieldElement {
        let other_inverse = other.inverse();

        // Use the multiplication method defined earlier
        self * &other_inverse
    }
}

impl From<&[u8]> for FieldElement {
    fn from(data: &[u8]) -> Self {
        let mut value: [u8; 32] = [0; 32];
        value.copy_from_slice(data);
        Self { value: value }
    }
}

impl From<u8> for FieldElement {
    fn from(data: u8) -> Self {
        let mut f = FieldElement::new();
        f.value[31] = data;
        f
    }
}

impl From<FieldElement> for u8 {
    fn from(fe: FieldElement) -> u8 {
        fe.value[31]
    }
}

// assumes usize < FieldElement (should be true, especially on the nano)
impl From<usize> for FieldElement {
    fn from(num: usize) -> Self {
        let mut f = FieldElement::new();
        let size_of_usize = core::mem::size_of::<usize>();
        let offset = if size_of_usize >= f.value.len() {
            0
        } else {
            f.value.len() - size_of_usize
        };

        for i in 0..size_of_usize {
            f.value[offset + i] = (num >> ((size_of_usize - 1 - i) * 8)) as u8;
        }

        f
    }
}

impl From<FieldElement> for usize {
    fn from(fe: FieldElement) -> usize {
        let mut value: usize = 0;
        let size_of_usize = core::mem::size_of::<usize>();
        let offset = if size_of_usize >= fe.value.len() {
            0
        } else {
            fe.value.len() - size_of_usize
        };

        for i in 0..size_of_usize {
            value |= (fe.value[i + offset] as usize) << ((size_of_usize - 1 - i) * 8);
        }

        value
    }
}

impl From<&FieldElement> for String<64> {
    fn from(f: &FieldElement) -> Self {
        let s: String<64> = f.value.into();
        s
    }
}

/// Maximum numbers of calls in a multicall Tx (out of memory)
/// NanoS = 3
/// NanoS+ = 10 (maybe more ?)
const MAX_TX_CALLS: usize = 10;

#[derive(Debug, Copy, Clone)]
pub enum AbstractCallData {
    Felt(FieldElement),
    Ref(usize),
    CallRef(usize, usize),
}

#[derive(Debug, Copy, Clone)]
pub struct AbstractCall {
    pub to: FieldElement,
    pub method: String<32>,
    pub selector: FieldElement,
    pub calldata: [AbstractCallData; 8],
    pub calldata_len: usize,
}

impl AbstractCall {
    pub fn new() -> Self {
        Self {
            to: FieldElement::new(),
            method: String::new(),
            selector: FieldElement::new(),
            calldata: [AbstractCallData::Felt(FieldElement::ZERO); 8],
            calldata_len: 0,
        }
    }

    pub fn clear(&mut self) {
        self.to.clear();
        self.method.clear();
        self.selector.clear();
        self.calldata
            .fill(AbstractCallData::Felt(FieldElement::ZERO));
        self.calldata_len = 0;
    }

    pub fn copy_from(&mut self, call: &Call) {
        self.to.copy_from(&call.to);
        self.method.copy_from(&call.method);
        self.selector.copy_from(&call.selector);
        for i in 0..call.calldata_len {
            let mut fe = FieldElement::new();
            fe.copy_from(&call.calldata[i]);
            self.calldata[i] = AbstractCallData::Felt(fe);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Call {
    pub to: FieldElement,
    pub method: String<32>,
    pub selector: FieldElement,
    pub calldata: [FieldElement; 16],
    pub calldata_len: usize,
}

impl Call {
    pub fn new() -> Self {
        Self {
            to: FieldElement::new(),
            method: String::new(),
            selector: FieldElement::new(),
            calldata: [FieldElement::ZERO; 16],
            calldata_len: 0,
        }
    }

    pub fn clear(&mut self) {
        self.to.clear();
        self.method.clear();
        self.selector.clear();
        self.calldata.fill(FieldElement::ZERO);
        self.calldata_len = 0;
    }
}

pub struct TransactionInfo {
    pub sender_address: FieldElement,
    pub max_fee: FieldElement,
    pub nonce: FieldElement,
    pub version: FieldElement,
    pub chain_id: FieldElement,
    pub callarray_len: FieldElement,
}

impl TransactionInfo {
    pub fn new() -> Self {
        Self {
            sender_address: FieldElement::new(),
            max_fee: FieldElement::new(),
            nonce: FieldElement::new(),
            version: FieldElement::new(),
            chain_id: FieldElement::new(),
            callarray_len: FieldElement::new(),
        }
    }

    pub fn clear(&mut self) {
        self.sender_address.clear();
        self.max_fee.clear();
        self.nonce.clear();
        self.version.clear();
        self.chain_id.clear();
        self.callarray_len.clear();
    }
}

pub struct Transaction {
    pub tx_info: TransactionInfo,
    pub calldata: [Call; MAX_TX_CALLS],
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            tx_info: TransactionInfo::new(),
            calldata: [Call::new(); MAX_TX_CALLS],
        }
    }

    pub fn clear(&mut self) {
        self.tx_info.clear();
        self.calldata.fill(Call::new());
    }
}
