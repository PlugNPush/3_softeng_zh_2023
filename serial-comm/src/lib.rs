//! Simple communication protocol over a serial connection, leveraging the COBS algorithm for byte
//! stuffing
//! (S. Cheshire and M. Baker, "Consistent overhead byte stuffing," in IEEE/ACM Transactions on Networking, vol. 7, no. 2, pp. 159-172, April 1999, doi: 10.1109/90.769765).

#![cfg_attr(not(test), no_std)]

pub mod cobs;
pub mod protocol;
